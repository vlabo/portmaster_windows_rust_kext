// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package Protocol

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type Packet struct {
	_tab flatbuffers.Table
}

func GetRootAsPacket(buf []byte, offset flatbuffers.UOffsetT) *Packet {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &Packet{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsPacket(buf []byte, offset flatbuffers.UOffsetT) *Packet {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &Packet{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *Packet) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *Packet) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *Packet) Id() uint32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return rcv._tab.GetUint32(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *Packet) MutateId(n uint32) bool {
	return rcv._tab.MutateUint32Slot(4, n)
}

func (rcv *Packet) ProcessId() *uint64 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		v := rcv._tab.GetUint64(o + rcv._tab.Pos)
		return &v
	}
	return nil
}

func (rcv *Packet) MutateProcessId(n uint64) bool {
	return rcv._tab.MutateUint64Slot(6, n)
}

func (rcv *Packet) ProcessPath() []byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(8))
	if o != 0 {
		return rcv._tab.ByteVector(o + rcv._tab.Pos)
	}
	return nil
}

func (rcv *Packet) Direction() byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(10))
	if o != 0 {
		return rcv._tab.GetByte(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *Packet) MutateDirection(n byte) bool {
	return rcv._tab.MutateByteSlot(10, n)
}

func (rcv *Packet) IpV6() bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(12))
	if o != 0 {
		return rcv._tab.GetBool(o + rcv._tab.Pos)
	}
	return false
}

func (rcv *Packet) MutateIpV6(n bool) bool {
	return rcv._tab.MutateBoolSlot(12, n)
}

func (rcv *Packet) Protocol() byte {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(14))
	if o != 0 {
		return rcv._tab.GetByte(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *Packet) MutateProtocol(n byte) bool {
	return rcv._tab.MutateByteSlot(14, n)
}

func (rcv *Packet) LocalIp(j int) uint32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(16))
	if o != 0 {
		a := rcv._tab.Vector(o)
		return rcv._tab.GetUint32(a + flatbuffers.UOffsetT(j*4))
	}
	return 0
}

func (rcv *Packet) LocalIpLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(16))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func (rcv *Packet) MutateLocalIp(j int, n uint32) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(16))
	if o != 0 {
		a := rcv._tab.Vector(o)
		return rcv._tab.MutateUint32(a+flatbuffers.UOffsetT(j*4), n)
	}
	return false
}

func (rcv *Packet) RemoteIp(j int) uint32 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(18))
	if o != 0 {
		a := rcv._tab.Vector(o)
		return rcv._tab.GetUint32(a + flatbuffers.UOffsetT(j*4))
	}
	return 0
}

func (rcv *Packet) RemoteIpLength() int {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(18))
	if o != 0 {
		return rcv._tab.VectorLen(o)
	}
	return 0
}

func (rcv *Packet) MutateRemoteIp(j int, n uint32) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(18))
	if o != 0 {
		a := rcv._tab.Vector(o)
		return rcv._tab.MutateUint32(a+flatbuffers.UOffsetT(j*4), n)
	}
	return false
}

func (rcv *Packet) LocalPort() uint16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(20))
	if o != 0 {
		return rcv._tab.GetUint16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *Packet) MutateLocalPort(n uint16) bool {
	return rcv._tab.MutateUint16Slot(20, n)
}

func (rcv *Packet) RemotePort() uint16 {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(22))
	if o != 0 {
		return rcv._tab.GetUint16(o + rcv._tab.Pos)
	}
	return 0
}

func (rcv *Packet) MutateRemotePort(n uint16) bool {
	return rcv._tab.MutateUint16Slot(22, n)
}

func PacketStart(builder *flatbuffers.Builder) {
	builder.StartObject(10)
}
func PacketAddId(builder *flatbuffers.Builder, id uint32) {
	builder.PrependUint32Slot(0, id, 0)
}
func PacketAddProcessId(builder *flatbuffers.Builder, processId uint64) {
	builder.PrependUint64(processId)
	builder.Slot(1)
}
func PacketAddProcessPath(builder *flatbuffers.Builder, processPath flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(2, flatbuffers.UOffsetT(processPath), 0)
}
func PacketAddDirection(builder *flatbuffers.Builder, direction byte) {
	builder.PrependByteSlot(3, direction, 0)
}
func PacketAddIpV6(builder *flatbuffers.Builder, ipV6 bool) {
	builder.PrependBoolSlot(4, ipV6, false)
}
func PacketAddProtocol(builder *flatbuffers.Builder, protocol byte) {
	builder.PrependByteSlot(5, protocol, 0)
}
func PacketAddLocalIp(builder *flatbuffers.Builder, localIp flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(6, flatbuffers.UOffsetT(localIp), 0)
}
func PacketStartLocalIpVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(4, numElems, 4)
}
func PacketAddRemoteIp(builder *flatbuffers.Builder, remoteIp flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(7, flatbuffers.UOffsetT(remoteIp), 0)
}
func PacketStartRemoteIpVector(builder *flatbuffers.Builder, numElems int) flatbuffers.UOffsetT {
	return builder.StartVector(4, numElems, 4)
}
func PacketAddLocalPort(builder *flatbuffers.Builder, localPort uint16) {
	builder.PrependUint16Slot(8, localPort, 0)
}
func PacketAddRemotePort(builder *flatbuffers.Builder, remotePort uint16) {
	builder.PrependUint16Slot(9, remotePort, 0)
}
func PacketEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}