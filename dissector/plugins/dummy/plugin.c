/*
 * Do not modify this file. Changes will be overwritten.
 *
 * Generated automatically from ../../tools/make-dissector-reg.py.
 */

#include "config.h"

#include <gmodule.h>

#include "moduleinfo.h"

/* plugins are DLLs */
#define WS_BUILD_DLL
#include "ws_symbol_export.h"

#ifndef ENABLE_STATIC
WS_DLL_PUBLIC_DEF void plugin_register (void);
WS_DLL_PUBLIC_DEF const gchar version[] = VERSION;

/* Start the functions we need for the plugin stuff */

extern void proto_register_dummy(void);

WS_DLL_PUBLIC_DEF void
plugin_register (void)
{
    void proto_register_dummy();
}

WS_DLL_PUBLIC_DEF void plugin_reg_handoff(void);

extern void proto_reg_handoff_dummy(void);

WS_DLL_PUBLIC_DEF void
plugin_reg_handoff(void)
{
    void proto_reg_handoff_dummy();
}
#endif