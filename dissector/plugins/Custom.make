#
# $Id: Custom.make.example 45142 2012-09-25 21:02:13Z jake $
#

_CUSTOM_SUBDIRS_ = \
        dummy

_CUSTOM_EXTRA_DIST_ = \
	Custom.m4 \
	Custom.make

_CUSTOM_plugin_ldadd_ = \
	-dlopen plugins/dummy/dummy.la
