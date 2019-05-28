#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.57])
AC_INIT([bzip2], [1.0.6], [Julian Seward <jseward@bzip.org>])
BZIP2_LT_CURRENT=1
BZIP2_LT_REVISION=6
BZIP2_LT_AGE=0
AC_CONFIG_SRCDIR([bzlib.h])
AC_CONFIG_MACRO_DIR([m4])

AM_INIT_AUTOMAKE([foreign subdir-objects])
AM_MAINTAINER_MODE

# Checks for programs.
AC_PROG_AWK
AC_PROG_CC_STDC
AC_PROG_CC_C_O
AC_USE_SYSTEM_EXTENSIONS
AC_PROG_INSTALL
AC_PROG_LN_S
AC_PROG_MAKE_SET
LT_INIT([disable-static pic-only])
PKG_PROG_PKG_CONFIG
gl_VISIBILITY
# Checks for libraries.

# Checks for header files.

# Checks for typedefs, structures, and compiler characteristics.

# Check for system features.
AC_SYS_LARGEFILE

AC_MSG_CHECKING([whether compiler understands -Wall])
save_CFLAGS="$CFLAGS"
CFLAGS="$CFLAGS -Wall"
AC_TRY_COMPILE([], [], [
	AC_MSG_RESULT([yes])
], [
	AC_MSG_RESULT([no])
	CFLAGS="$save_CFLAGS"
])

AC_MSG_CHECKING([whether compiler understands -Winline])
save_CFLAGS="$CFLAGS"
CFLAGS="$CFLAGS -Winline"
AC_TRY_COMPILE([], [], [
	AC_MSG_RESULT([yes])
], [
	AC_MSG_RESULT([no])
	CFLAGS="$save_CFLAGS"
])

dnl === Rust compiler ==========================================================

AC_CHECK_PROGS(CARGO, [cargo], [no])
AS_IF(test x$CARGO = xno,
    AC_MSG_ERROR([cargo is required.  Please install the Rust toolchain from https://www.rust-lang.org/])
)
AC_CHECK_PROGS(RUSTC, [rustc], [no])
AS_IF(test x$RUSTC = xno,
    AC_MSG_ERROR([rustc is required.  Please install the Rust toolchain from https://www.rust-lang.org/])
)

dnl Minimum version of rustc that we support
dnl If you change this, please update COMPILING.md
MINIMUM_RUST_MAJOR=1
MINIMUM_RUST_MINOR=31

rust_version=`$RUSTC --version`
version=`echo "$rust_version" | sed -e 's/^rustc //g'`
major=`echo "$version" | cut -d. -f1`
minor=`echo "$version" | cut -d. -f2`

AS_IF(test "$major" -lt "$MINIMUM_RUST_MAJOR" -o "$minor" -lt "$MINIMUM_RUST_MINOR",
    AC_MSG_ERROR([librsvg needs at rustc version at least $MINIMUM_RUST_MAJOR.$MINIMUM_RUST_MINOR])
)

dnl ===========================================================================
dnl Specify --enable-debug to make a development release.  By default,
dnl we build in public release mode.

AC_ARG_ENABLE(debug,
              AC_HELP_STRING([--enable-debug],
                             [Build Rust code with debugging information [default=no]]),
              [debug_release=$enableval],
	      [
	        if test "x$LIBRSVG_DEBUG" = "x"; then
		  debug_release="no"
		elif test "x$LIBRSVG_DEBUG" = "xyes"; then
		  debug_release="yes"
		elif test "x$LIBRSVG_DEBUG" = "xno"; then
		  debug_release="no"
		fi
	      ])

AC_MSG_CHECKING(whether to build Rust code with debugging information)
if test "x$debug_release" = "xyes" ; then
	AC_MSG_RESULT(yes)
	RUST_TARGET_SUBDIR=debug
else
	AC_MSG_RESULT(no)
	RUST_TARGET_SUBDIR=release
fi
AM_CONDITIONAL([DEBUG_RELEASE], [test "x$debug_release" = "xyes"])

AC_ARG_VAR(RUST_TARGET, [Set rust target (default=$host)])
if test "x$RUST_TARGET" = x; then
	RUST_TARGET=$host
fi

AM_CONDITIONAL(CROSS_COMPILING, test $cross_compiling = yes)
if test "x$cross_compiling" = "xyes" ; then
	RUST_TARGET_SUBDIR="$RUST_TARGET/$RUST_TARGET_SUBDIR"
fi
AC_SUBST([RUST_TARGET_SUBDIR])

dnl ===========================================================================


# Checks for library functions.

# Write the output.
AC_SUBST([BZIP2_LT_CURRENT])
AC_SUBST([BZIP2_LT_REVISION])
AC_SUBST([BZIP2_LT_AGE])
AC_CONFIG_FILES([Makefile bzip2.pc])
AC_OUTPUT
