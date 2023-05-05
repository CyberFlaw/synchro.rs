AC_INIT([audioinput], [m4_esyscmd_s([cat version])], [tcr19cs029@gectcr.ac.in])
AM_INIT_AUTOMAKE([subdir-objects -Wall -Werror foreign])
dnl AC_CONFIG_MACRO_DIRS([m4])
AM_PROG_AR
LT_INIT
AC_PROG_CC

AM_PROG_LIBTOOL


dnl ############################
dnl checking if debug is enabled
dnl ############################

AC_ARG_ENABLE([debug],
  AS_HELP_STRING([--enable-debug],
    [enable debug messages and frequency table output])
)

AS_IF([test "x$enable_debug" != "xyes"], [
  dnl enabling debug mode
  CPPFLAGS="$CPPFLAGS -DNDEBUG"
])

AC_ARG_ENABLE([asan],
  AS_HELP_STRING([--enable-asan],
    [build with AddressSanitizer])
)

AS_IF([test "x$enable_asan" = "xyes"], [
  dnl enabling asan
  CPPFLAGS="$CPPFLAGS -fsanitize=address"
  LDFLAGS="$LDFLAGS -fsanitize=address"
])

AC_ARG_ENABLE([tsan],
  AS_HELP_STRING([--enable-tsan],
    [build with ThreadSanitizer])
)

AS_IF([test "x$enable_tsan" = "xyes"], [
  dnl enabling tsan
  CPPFLAGS="$CPPFLAGS -fsanitize=thread"
  LDFLAGS="$LDFLAGS -fsanitize=thread"
])

AC_ARG_ENABLE([ubsan],
  AS_HELP_STRING([--enable-ubsan],
    [build with UndefinedBehaviorSanitizer])
)

AS_IF([test "x$enable_ubsan" = "xyes"], [
  dnl enabling ubsan
  CPPFLAGS="$CPPFLAGS -fsanitize=undefined"
  LDFLAGS="$LDFLAGS -fsanitize=undefined"
])


dnl ######################
dnl checking for pthread
dnl ######################

AC_CHECK_HEADERS([pthread.h],
	AC_CHECK_LIB(pthread, pthread_create, LIBS="$LIBS -lpthread",
		AC_MSG_ERROR([pthread.h found but there is no pthread library to make use of])
	),
	AC_MSG_ERROR([no pthread.h header header file found])
)

dnl ######################
dnl checking for alloca.h
dnl ######################

AC_CHECK_HEADER([alloca.h], [CPPFLAGS="$CPPFLAGS -DHAVE_ALLOCA_H"])

dnl ######################
dnl checking for alsa dev
dnl ######################
AC_ARG_ENABLE([input_alsa],
  AS_HELP_STRING([--disable-input-alsa],
    [do not include support for input from alsa streams])
)

AS_IF([test "x$enable_input_alsa" != "xno"], [
  AC_CHECK_LIB(asound, snd_pcm_open, have_alsa=yes, have_alsa=no)
  if [[ $have_alsa = "yes" ]] ; then
    LIBS="$LIBS -lasound"
    CPPFLAGS="$CPPFLAGS -DALSA"
  fi
  if [[ $have_alsa = "no" ]] ; then
    AC_MSG_NOTICE([WARNING: No alsa dev files found building without alsa support])
  fi],
  [have_alsa=no]
)

AM_CONDITIONAL([ALSA], [test "x$have_alsa" = "xyes"])


dnl ######################
dnl checking for pulse dev
dnl ######################
AC_ARG_ENABLE([input_pulse],
  AS_HELP_STRING([--disable-input-pulse],
    [do not include support for input from pulseaudio])
)

AS_IF([test "x$enable_input_pulse" != "xno"], [
  AC_CHECK_LIB(pulse-simple, pa_simple_new, have_pulse=yes, have_pulse=no)
  if [[ $have_pulse = "yes" ]] ; then
    LIBS="$LIBS -lpulse-simple -lpulse"
    CPPFLAGS="$CPPFLAGS -DPULSE"
  fi

  if [[ $have_pulse = "no" ]] ; then
    AC_MSG_NOTICE([WARNING: No pulseaudio dev files found building without pulseaudio support])
  fi],
  [have_pulse=no]
)

AM_CONDITIONAL([PULSE], [test "x$have_pulse" = "xyes"])

build_linux=no
build_windows=no
build_mac=no

AC_MSG_NOTICE([Checking OS])
# Detect the target system
case "${host_os}" in
    linux*)
        AC_MSG_NOTICE([Linux detected])
        build_linux=yes
        ;;
    darwin*)
        AC_MSG_NOTICE([OSX detected])
        build_mac=yes
        ;;
    freebsd*)
        AC_MSG_NOTICE([FreeBSD detected])
        build_linux=yes
        ;;
    *)
        AC_MSG_ERROR(["OS $host_os is not supported"])
        ;;
esac

# Pass the conditionals to automake
AM_CONDITIONAL([LINUX], [test "$build_linux" = "yes"])
AM_CONDITIONAL([OSX], [test "$build_mac" = "yes"])

AC_CONFIG_FILES([Makefile])
AC_OUTPUT





