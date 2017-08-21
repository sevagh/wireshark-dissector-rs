#include "config.h"

#include <epan/packet.h>

#define DUMMY_PORT 8888

static int proto_dummy = -1;

static gint ett_dummy = -1;

static int dissect_dummy(tvbuff_t *tvb, packet_info *pinfo, proto_tree *tree, void *data)
{
   ((void) data); // suppress compile warning.

   col_set_str(pinfo->cinfo, COL_PROTOCOL, "DUMMY");
   col_clear(pinfo->cinfo,COL_INFO);

   if (tree) { /* we are being asked for details */
      proto_item *ti;
      DummyPacket *msg;

      ti = proto_tree_add_item(tree, proto_dummy, tvb, 0, -1, FALSE);
      tree = proto_item_add_subtree(ti, ett_dummy);
      msg = (DummyPacket *) wmem_alloc(wmem_packet_scope(), sizeof(DummyPacket));
      DummyPacket_dissect(msg, proto_dummy, tvb, 0, pinfo, tree);
   }

   return tvb_captured_length(tvb);
}

void proto_register_dummy(void)
{
   static int *ett[] = { &ett_dummy };

   proto_dummy = proto_register_protocol
      (
         "Dummy Protocol", /* name */
         "DUMMY",          /* short name */
         "dummy"           /* abbrev */
      );

   DummyPacket_register(proto_dummy);
   proto_register_subtree_array(ett, array_length(ett));
}

void proto_reg_handoff_dummy(void)
{
   static dissector_handle_t dummy_handle;

   dummy_handle = create_dissector_handle(dissect_dummy, proto_dummy);
   dissector_add_uint("tcp.port", DUMMY_PORT, dummy_handle);
}
