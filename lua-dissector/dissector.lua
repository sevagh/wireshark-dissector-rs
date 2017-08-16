require "bit32"
require "string"

-- USAGE
-- tshark -r dump.pcap -X lua_script:tools/wireshark/dissector.lua

dummy_proto = Proto("dummy","Dummy Protocol")
local dummyproto_default_port = 8888

dummy_proto.fields = { f_version, f_body }

function decode_content(buf)
    buf_ = {}
    for idx, single_byte in ipairs(buf) do
	buf_[idx] = single_byte:uint() - 1
    end
    return tostring(buf_)
end

function dummy_proto.dissector(tvb, pktinfo, tree)
    -- invalid packet
    if tvb:len() ~= 16 then return end

    pktinfo.cols.protocol = dummy_proto.name

    -- request
    local version = decode_content(tvb)

    local root = tree:add(dummy_proto, buf() ,"Dummy Protocol Data")
    root:add(f_version, version, "version: " .. version)
    root:add(f_body, body, "body: " .. body)

    pktinfo.cols.info:append('VER=' .. version)
    pktinfo.cols.info:append(' BODY=' .. body)
end

tcp_table = DissectorTable.get("tcp.port")
tcp_table:add(dummyproto_default_port, dummy_proto)
