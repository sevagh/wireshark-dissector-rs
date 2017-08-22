#include "config.h"

#include <epan/dissectors/packet-tcp.h>
#include <epan/packet.h>
#include <epan/tvbuff-int.h>
#include <epan/tvbuff.h>
#include <wiretap/wtap.h>

#include <glib.h>
#include <stdint.h>
#include <stdio.h>

#include "./packet-dummy.h"

#define DUMMY_PORT 8888

static int proto_dummy = -1;
static gint ett_dummy = -1;
static int hf_dummy_version = -1;
static int hf_dummy_body = -1;

static int
dissect_dummy(tvbuff_t *tvb, packet_info *pinfo, proto_tree *tree, void *data);

extern int32_t
dissect_dummy_rs(const void *data);

void
proto_register_dummy(void);
void
proto_reg_handoff_dummy(void);

static int
dissect_dummy(tvbuff_t *tvb, packet_info *pinfo, proto_tree *tree, void *data)
{
	((void)data); // suppress compile warning.

	col_set_str(pinfo->cinfo, COL_PROTOCOL, "DUMMY");
	col_clear(pinfo->cinfo, COL_INFO);

	dissect_dummy_rs(tvb->real_data);

	gint offset = 0;

	if (tree) {
		proto_item *ti;
		ti = proto_tree_add_item(tree, proto_dummy, tvb, 0, -1, FALSE);
		tree = proto_item_add_subtree(ti, ett_dummy);
		proto_tree_add_item(tree, hf_dummy_version, tvb, offset, 8, FT_STRING);
		offset += 8;
		proto_tree_add_item(tree, hf_dummy_body, tvb, offset, 8, FT_STRING);
	}

	return tvb_captured_length(tvb);
}

void
proto_register_dummy(void)
{
	static int *ett[] = {&ett_dummy};

	proto_dummy = proto_register_protocol("Dummy Protocol", /* name */
	    "DUMMY",                                            /* short name */
	    "dummy"                                             /* abbrev */
	    );

	static hf_register_info hf[] = {
	    {&hf_dummy_version, {"Version", "dummy.version", FT_STRING, BASE_NONE,
	                            NULL, 0x0, NULL, HFILL}},
	    {&hf_dummy_body, {"Body", "dummy.body", FT_STRING, BASE_NONE, NULL, 0x0,
	                         NULL, HFILL}},
	};

	proto_register_field_array(proto_dummy, hf, array_length(hf));
	proto_register_subtree_array(ett, array_length(ett));
}

void
proto_reg_handoff_dummy(void)
{
	static dissector_handle_t dummy_handle;

	dummy_handle = create_dissector_handle(dissect_dummy, proto_dummy);
	dissector_add_uint_with_preference("tcp.port", DUMMY_PORT, dummy_handle);
}
