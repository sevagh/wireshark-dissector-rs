# Makefile.am
#

include $(top_srcdir)/Makefile.am.inc
include ../Makefile.am.inc

# the name of the plugin
PLUGIN_NAME = dummy

# Non-generated sources to be scanned for registration routines
NONGENERATED_REGISTER_C_FILES = \
	packet-dummy.c

plugin_LTLIBRARIES = dummy.la

dummy_la_SOURCES = \
	plugin.c \
	moduleinfo.h \
	$(SRC_FILES)	\
	$(HEADER_FILES)

dummy_la_CPPFLAGS = $(AM_CPPFLAGS) $(PLUGIN_CPPFLAGS)

dummy_la_CFLAGS = $(AM_CFLAGS) $(PLUGIN_CFLAGS)

dummy_la_LDFLAGS = $(PLUGIN_LDFLAGS)

CLEANFILES = \
	dummy \
	*~

MAINTAINERCLEANFILES = \
	Makefile.in	\
	plugin.c

EXTRA_DIST = \
	plugin.rc.in		\
	CMakeLists.txt
