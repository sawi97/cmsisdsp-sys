# This is an example Makefile to show how to build the library

COMPILER_ROOT = /usr/bin
CMSIS_ROOT = cmsis

# Compilation tools
#CC := $(COMPILER_ROOT)/ARMCompiler6.18/bin/armclang
#ARMAR :=  $(COMPILER_ROOT)/ARMCompiler6.18/bin/armar

#CC := $(COMPILER_ROOT)/clang
#ARMAR :=  $(COMPILER_ROOT)/llvm-ar

CC := $(COMPILER_ROOT)/arm-none-eabi-gcc
ARMAR :=  $(COMPILER_ROOT)/arm-none-eabi-ar

# Compilation flags (here for Cortex-M33)
CFLAGS := \
 -mcpu=cortex-m33 -mfpu=fpv5-sp-d16 -mfloat-abi=hard \
 -Wsign-compare \
 -Wdouble-promotion \
 -Ofast -ffast-math -s \
 -DNDEBUG \
 -DDISABLEFLOAT16 \
 -munaligned-access \
 -fshort-enums -fshort-wchar \
 -Wall -Werror \
 -D__ICCARM__ \
 -D__ARMVFP__
 # |\ Enables VSQRT usage (for some reason restricted to IAR)

EXTRAFLAGS := -DDISABLEFLOAT16 -DARM_DSP_CONFIG_TABLES -DARM_FAST_ALLOW_TABLES -DARM_FFT_ALLOW_TABLES \
                               -DARM_MATH_LOOPUNROLL -DDISABLEFLOAT16 -DARM_TABLE_SIN_F32

CFLAGS := $(CFLAGS) $(EXTRAFLAGS)

# Path to CMSIS_5
CMSIS_5 := $(CMSIS_ROOT)/CMSIS_5

# Path to CMSIS_DSP
CMSIS_DSP := $(CMSIS_ROOT)/CMSIS-DSP

# Path to CMSIS Core includes for Cortex-M
# For low end Cortex-A, use Core_A
# For high end Cortex-A (aarch64), don't use CMSIS
# Core Includes (Refer to the CMSIS-DSP README to
# know how to build in that case)
CMSIS_CORE_INCLUDES := $(CMSIS_5)/CMSIS/Core/Include

# Sources
SRCS := $(CMSIS_DSP)/Source/BasicMathFunctions/BasicMathFunctions.c \
 $(CMSIS_DSP)/Source/CommonTables/CommonTables.c \
 $(CMSIS_DSP)/Source/InterpolationFunctions/InterpolationFunctions.c \
 $(CMSIS_DSP)/Source/BayesFunctions/BayesFunctions.c \
 $(CMSIS_DSP)/Source/MatrixFunctions/MatrixFunctions.c \
 $(CMSIS_DSP)/Source/ComplexMathFunctions/ComplexMathFunctions.c \
 $(CMSIS_DSP)/Source/QuaternionMathFunctions/QuaternionMathFunctions.c \
 $(CMSIS_DSP)/Source/ControllerFunctions/ControllerFunctions.c \
 $(CMSIS_DSP)/Source/SVMFunctions/SVMFunctions.c \
 $(CMSIS_DSP)/Source/DistanceFunctions/DistanceFunctions.c \
 $(CMSIS_DSP)/Source/StatisticsFunctions/StatisticsFunctions.c \
 $(CMSIS_DSP)/Source/FastMathFunctions/FastMathFunctions.c \
 $(CMSIS_DSP)/Source/SupportFunctions/SupportFunctions.c \
 $(CMSIS_DSP)/Source/FilteringFunctions/FilteringFunctions.c \
 $(CMSIS_DSP)/Source/TransformFunctions/TransformFunctions.c

# Includes
DSP_INCLUDES := $(CMSIS_DSP)/Include \
  $(CMSIS_DSP)/PrivateInclude

# If Neon and Cortex-A
#DSP_INCLUDES += $(CMSIS_DSP)/ComputeLibrary/Include
#SRCS += $(CMSIS_DSP)/ComputeLibrary/Source/arm_cl_tables.c

# Compilation flags for include folders
INC_FLAGS := $(addprefix -I,$(DSP_INCLUDES))
INC_FLAGS += $(addprefix -I,$(CMSIS_CORE_INCLUDES))
CFLAGS += $(INC_FLAGS)

# Output folder for build products
BUILDDIR := ./builddir

OBJECTS := $(SRCS:%=$(BUILDDIR)/%.o)

# Build rules
$(BUILDDIR)/libCMSISDSP.a: $(OBJECTS)
	$(ARMAR) -rc $@ $(OBJECTS)

$(BUILDDIR)/%.c.o: %.c
	mkdir -p $(dir $@)
	$(CC) -c $(CFLAGS) $(CPPFLAGS) $< -o $@

clean:
	rm -r builddir
