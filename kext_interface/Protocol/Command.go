// Code generated by the FlatBuffers compiler. DO NOT EDIT.

package Protocol

import (
	flatbuffers "github.com/google/flatbuffers/go"
)

type Command struct {
	_tab flatbuffers.Table
}

func GetRootAsCommand(buf []byte, offset flatbuffers.UOffsetT) *Command {
	n := flatbuffers.GetUOffsetT(buf[offset:])
	x := &Command{}
	x.Init(buf, n+offset)
	return x
}

func GetSizePrefixedRootAsCommand(buf []byte, offset flatbuffers.UOffsetT) *Command {
	n := flatbuffers.GetUOffsetT(buf[offset+flatbuffers.SizeUint32:])
	x := &Command{}
	x.Init(buf, n+offset+flatbuffers.SizeUint32)
	return x
}

func (rcv *Command) Init(buf []byte, i flatbuffers.UOffsetT) {
	rcv._tab.Bytes = buf
	rcv._tab.Pos = i
}

func (rcv *Command) Table() flatbuffers.Table {
	return rcv._tab
}

func (rcv *Command) CommandType() CommandUnion {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(4))
	if o != 0 {
		return CommandUnion(rcv._tab.GetByte(o + rcv._tab.Pos))
	}
	return 0
}

func (rcv *Command) MutateCommandType(n CommandUnion) bool {
	return rcv._tab.MutateByteSlot(4, byte(n))
}

func (rcv *Command) Command(obj *flatbuffers.Table) bool {
	o := flatbuffers.UOffsetT(rcv._tab.Offset(6))
	if o != 0 {
		rcv._tab.Union(obj, o)
		return true
	}
	return false
}

func CommandStart(builder *flatbuffers.Builder) {
	builder.StartObject(2)
}
func CommandAddCommandType(builder *flatbuffers.Builder, commandType CommandUnion) {
	builder.PrependByteSlot(0, byte(commandType), 0)
}
func CommandAddCommand(builder *flatbuffers.Builder, command flatbuffers.UOffsetT) {
	builder.PrependUOffsetTSlot(1, flatbuffers.UOffsetT(command), 0)
}
func CommandEnd(builder *flatbuffers.Builder) flatbuffers.UOffsetT {
	return builder.EndObject()
}