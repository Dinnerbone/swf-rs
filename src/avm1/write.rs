#![allow(clippy::cognitive_complexity, clippy::unreadable_literal)]

use crate::avm1::opcode::OpCode;
use crate::avm1::types::*;
use crate::write::SwfWrite;
use std::io::{Result, Write};

pub struct Writer<W: Write> {
    inner: W,
    version: u8,
}

impl<W: Write> SwfWrite<W> for Writer<W> {
    fn get_inner(&mut self) -> &mut W {
        &mut self.inner
    }
}

impl<W: Write> Writer<W> {
    pub fn new(inner: W, version: u8) -> Writer<W> {
        Writer { inner, version }
    }

    pub fn write_action_list(&mut self, actions: &[Action]) -> Result<()> {
        for action in actions {
            self.write_action(action)?;
        }
        self.write_u8(0)?; // End
        Ok(())
    }

    pub fn write_action(&mut self, action: &Action) -> Result<()> {
        match *action {
            Action::Add => self.write_action_header(OpCode::Add, 0)?,
            Action::Add2 => self.write_action_header(OpCode::Add2, 0)?,
            Action::And => self.write_action_header(OpCode::And, 0)?,
            Action::AsciiToChar => self.write_action_header(OpCode::AsciiToChar, 0)?,
            Action::BitAnd => self.write_action_header(OpCode::BitAnd, 0)?,
            Action::BitLShift => self.write_action_header(OpCode::BitLShift, 0)?,
            Action::BitOr => self.write_action_header(OpCode::BitOr, 0)?,
            Action::BitRShift => self.write_action_header(OpCode::BitRShift, 0)?,
            Action::BitURShift => self.write_action_header(OpCode::BitURShift, 0)?,
            Action::BitXor => self.write_action_header(OpCode::BitXor, 0)?,
            Action::Call => self.write_action_header(OpCode::Call, 0)?,
            Action::CallFunction => self.write_action_header(OpCode::CallFunction, 0)?,
            Action::CallMethod => self.write_action_header(OpCode::CallMethod, 0)?,
            Action::CastOp => self.write_action_header(OpCode::CastOp, 0)?,
            Action::CharToAscii => self.write_action_header(OpCode::CharToAscii, 0)?,
            Action::CloneSprite => self.write_action_header(OpCode::CloneSprite, 0)?,
            Action::ConstantPool(ref constants) => {
                let len = 2 + constants.iter().map(|c| c.len() + 1).sum::<usize>();
                self.write_action_header(OpCode::ConstantPool, len)?;
                self.write_u16(constants.len() as u16)?;
                for constant in constants {
                    self.write_c_string(constant)?;
                }
            }
            Action::Decrement => self.write_action_header(OpCode::Decrement, 0)?,
            Action::DefineFunction {
                ref name,
                ref params,
                ref actions,
            } => {
                let mut action_buf = vec![];
                {
                    let mut fn_writer = Writer::new(&mut action_buf, self.version);
                    fn_writer.write_action_list(actions)?;
                }
                let len = name.len()
                    + 1
                    + 2
                    + params.iter().map(|p| p.len() + 1).sum::<usize>()
                    + 2
                    + action_buf.len();
                self.write_action_header(OpCode::DefineFunction, len)?;
                self.write_c_string(name)?;
                self.write_u16(params.len() as u16)?;
                for param in params {
                    self.write_c_string(param)?;
                }
                self.write_u16(action_buf.len() as u16)?;
                self.inner.write_all(&action_buf)?;
            }
            Action::DefineFunction2(ref function) => {
                let mut action_buf = vec![];
                {
                    let mut fn_writer = Writer::new(&mut action_buf, self.version);
                    fn_writer.write_action_list(&function.actions)?;
                }
                let len = function.name.len()
                    + 1
                    + 3
                    + function
                        .params
                        .iter()
                        .map(|p| p.name.len() + 2)
                        .sum::<usize>()
                    + 4
                    + action_buf.len();
                let num_registers = function
                    .params
                    .iter()
                    .map(|p| if p.register_index.is_none() { 1 } else { 0 })
                    .sum();
                self.write_action_header(OpCode::DefineFunction2, len)?;
                self.write_c_string(&function.name)?;
                self.write_u16(function.params.len() as u16)?;
                self.write_u8(num_registers)?;
                let flags = if function.preload_global {
                    0b1_00000000
                } else {
                    0
                } | if function.preload_parent {
                    0b10000000
                } else {
                    0
                } | if function.preload_root { 0b1000000 } else { 0 }
                    | if function.suppress_super { 0b100000 } else { 0 }
                    | if function.preload_super { 0b10000 } else { 0 }
                    | if function.suppress_arguments {
                        0b1000
                    } else {
                        0
                    }
                    | if function.preload_arguments { 0b100 } else { 0 }
                    | if function.suppress_this { 0b10 } else { 0 }
                    | if function.preload_this { 0b1 } else { 0 };
                self.write_u16(flags)?;
                for param in &function.params {
                    self.write_u8(if let Some(n) = param.register_index {
                        n
                    } else {
                        0
                    })?;
                    self.write_c_string(&param.name)?;
                }
                self.write_u16(action_buf.len() as u16)?;
                self.inner.write_all(&action_buf)?;
            }
            Action::DefineLocal => self.write_action_header(OpCode::DefineLocal, 0)?,
            Action::DefineLocal2 => self.write_action_header(OpCode::DefineLocal2, 0)?,
            Action::Divide => self.write_action_header(OpCode::Divide, 0)?,
            Action::Delete => self.write_action_header(OpCode::Delete, 0)?,
            Action::Delete2 => self.write_action_header(OpCode::Delete2, 0)?,
            Action::EndDrag => self.write_action_header(OpCode::EndDrag, 0)?,
            Action::Enumerate => self.write_action_header(OpCode::Enumerate, 0)?,
            Action::Enumerate2 => self.write_action_header(OpCode::Enumerate2, 0)?,
            Action::Equals => self.write_action_header(OpCode::Equals, 0)?,
            Action::Equals2 => self.write_action_header(OpCode::Equals2, 0)?,
            Action::Extends => self.write_action_header(OpCode::Extends, 0)?,
            Action::GetMember => self.write_action_header(OpCode::GetMember, 0)?,
            Action::GetProperty => self.write_action_header(OpCode::GetProperty, 0)?,
            Action::GetTime => self.write_action_header(OpCode::GetTime, 0)?,
            Action::GetUrl {
                ref url,
                ref target,
            } => {
                self.write_action_header(OpCode::GetUrl, url.len() + target.len() + 2)?;
                self.write_c_string(url)?;
                self.write_c_string(target)?;
            }
            Action::GetUrl2 {
                send_vars_method,
                is_target_sprite,
                is_load_vars,
            } => {
                self.write_action_header(OpCode::GetUrl2, 1)?;
                let flags = (match send_vars_method {
                    SendVarsMethod::None => 0,
                    SendVarsMethod::Get => 1,
                    SendVarsMethod::Post => 2,
                } << 6)
                    | if is_target_sprite { 0b10 } else { 0 }
                    | if is_load_vars { 0b1 } else { 0 };
                self.write_u8(flags)?;
            }
            Action::GetVariable => self.write_action_header(OpCode::GetVariable, 0)?,
            Action::GotoFrame(frame) => {
                self.write_action_header(OpCode::GotoFrame, 2)?;
                self.write_u16(frame)?;
            }
            Action::GotoFrame2 {
                set_playing,
                scene_offset,
            } => {
                if scene_offset != 0 {
                    self.write_action_header(OpCode::GotoFrame2, 3)?;
                    self.write_u8(if set_playing { 0b11 } else { 0b01 })?;
                    self.write_u16(scene_offset)?;
                } else {
                    self.write_action_header(OpCode::GotoFrame2, 1)?;
                    self.write_u8(if set_playing { 0b10 } else { 0b00 })?;
                }
            }
            Action::GotoLabel(ref label) => {
                self.write_action_header(OpCode::GotoLabel, label.len() + 1)?;
                self.write_c_string(label)?;
            }
            Action::Greater => self.write_action_header(OpCode::Greater, 0)?,
            Action::If { offset } => {
                self.write_action_header(OpCode::If, 2)?;
                self.write_i16(offset)?;
            }
            Action::ImplementsOp => self.write_action_header(OpCode::ImplementsOp, 0)?,
            Action::Increment => self.write_action_header(OpCode::Increment, 0)?,
            Action::InitArray => self.write_action_header(OpCode::InitArray, 0)?,
            Action::InitObject => self.write_action_header(OpCode::InitObject, 0)?,
            Action::InstanceOf => self.write_action_header(OpCode::InstanceOf, 0)?,
            Action::Jump { offset } => {
                self.write_action_header(OpCode::Jump, 2)?;
                self.write_i16(offset)?;
            }
            Action::Less => self.write_action_header(OpCode::Less, 0)?,
            Action::Less2 => self.write_action_header(OpCode::Less2, 0)?,
            Action::MBAsciiToChar => self.write_action_header(OpCode::MBAsciiToChar, 0)?,
            Action::MBCharToAscii => self.write_action_header(OpCode::MBCharToAscii, 0)?,
            Action::MBStringExtract => self.write_action_header(OpCode::MBStringExtract, 0)?,
            Action::MBStringLength => self.write_action_header(OpCode::MBStringLength, 0)?,
            Action::Modulo => self.write_action_header(OpCode::Modulo, 0)?,
            Action::Multiply => self.write_action_header(OpCode::Multiply, 0)?,
            Action::NewMethod => self.write_action_header(OpCode::NewMethod, 0)?,
            Action::NewObject => self.write_action_header(OpCode::NewObject, 0)?,
            Action::NextFrame => self.write_action_header(OpCode::NextFrame, 0)?,
            Action::Not => self.write_action_header(OpCode::Not, 0)?,
            Action::Or => self.write_action_header(OpCode::Or, 0)?,
            Action::Play => self.write_action_header(OpCode::Play, 0)?,
            Action::Pop => self.write_action_header(OpCode::Pop, 0)?,
            Action::PreviousFrame => self.write_action_header(OpCode::PreviousFrame, 0)?,
            Action::Push(ref values) => {
                let len = values
                    .iter()
                    .map(|v| match *v {
                        Value::Str(ref string) => string.len() + 2,
                        Value::Null | Value::Undefined => 1,
                        Value::Register(_) | Value::Bool(_) => 2,
                        Value::Double(_) => 9,
                        Value::Float(_) | Value::Int(_) => 5,
                        Value::ConstantPool(v) => {
                            if v < 256 {
                                2
                            } else {
                                3
                            }
                        }
                    })
                    .sum();
                self.write_action_header(OpCode::Push, len)?;
                for value in values {
                    self.write_push_value(value)?;
                }
            }
            Action::PushDuplicate => self.write_action_header(OpCode::PushDuplicate, 0)?,
            Action::RandomNumber => self.write_action_header(OpCode::RandomNumber, 0)?,
            Action::RemoveSprite => self.write_action_header(OpCode::RemoveSprite, 0)?,
            Action::Return => self.write_action_header(OpCode::Return, 0)?,
            Action::SetMember => self.write_action_header(OpCode::SetMember, 0)?,
            Action::SetProperty => self.write_action_header(OpCode::SetProperty, 0)?,
            Action::SetTarget(ref target) => {
                self.write_action_header(OpCode::SetTarget, target.len() + 1)?;
                self.write_c_string(target)?;
            }
            Action::SetTarget2 => self.write_action_header(OpCode::SetTarget2, 0)?,
            Action::SetVariable => self.write_action_header(OpCode::SetVariable, 0)?,
            Action::StackSwap => self.write_action_header(OpCode::StackSwap, 0)?,
            Action::StartDrag => self.write_action_header(OpCode::StartDrag, 0)?,
            Action::Stop => self.write_action_header(OpCode::Stop, 0)?,
            Action::StopSounds => self.write_action_header(OpCode::StopSounds, 0)?,
            Action::StoreRegister(register) => {
                self.write_action_header(OpCode::StoreRegister, 1)?;
                self.write_u8(register)?;
            }
            Action::StrictEquals => self.write_action_header(OpCode::StrictEquals, 0)?,
            Action::StringAdd => self.write_action_header(OpCode::StringAdd, 0)?,
            Action::StringEquals => self.write_action_header(OpCode::StringEquals, 0)?,
            Action::StringExtract => self.write_action_header(OpCode::StringExtract, 0)?,
            Action::StringGreater => self.write_action_header(OpCode::StringGreater, 0)?,
            Action::StringLength => self.write_action_header(OpCode::StringLength, 0)?,
            Action::StringLess => self.write_action_header(OpCode::StringLess, 0)?,
            Action::Subtract => self.write_action_header(OpCode::Subtract, 0)?,
            Action::TargetPath => self.write_action_header(OpCode::TargetPath, 0)?,
            Action::Throw => self.write_action_header(OpCode::Throw, 0)?,
            Action::ToggleQuality => self.write_action_header(OpCode::ToggleQuality, 0)?,
            Action::ToInteger => self.write_action_header(OpCode::ToInteger, 0)?,
            Action::ToNumber => self.write_action_header(OpCode::ToNumber, 0)?,
            Action::ToString => self.write_action_header(OpCode::ToString, 0)?,
            Action::Trace => self.write_action_header(OpCode::Trace, 0)?,
            Action::Try(ref try_block) => {
                let try_length;
                let catch_length;
                let finally_length;
                let mut action_buf = vec![];
                {
                    let mut fn_writer = Writer::new(&mut action_buf, self.version);
                    fn_writer.write_action_list(&try_block.try_actions)?;
                    try_length = fn_writer.inner.len();
                    if let Some((_, ref catch)) = try_block.catch {
                        fn_writer.write_action_list(catch)?;
                    }
                    catch_length = fn_writer.inner.len() - try_length;
                    if let Some(ref finally) = try_block.finally {
                        fn_writer.write_action_list(finally)?;
                    }
                    finally_length = fn_writer.inner.len() - (try_length + catch_length);
                }
                let len = 7
                    + action_buf.len()
                    + if let Some((CatchVar::Var(ref name), _)) = try_block.catch {
                        name.len() + 1
                    } else {
                        1
                    };
                self.write_action_header(OpCode::Try, len)?;
                self.write_u8(
                    if let Some((CatchVar::Register(_), _)) = try_block.catch {
                        0b100
                    } else {
                        0
                    } | if try_block.finally.is_some() { 0b10 } else { 0 }
                        | if try_block.catch.is_some() { 0b1 } else { 0 },
                )?;
                self.write_u16(try_length as u16)?;
                self.write_u16(catch_length as u16)?;
                self.write_u16(finally_length as u16)?;
                match try_block.catch {
                    Some((CatchVar::Var(ref name), _)) => self.write_c_string(name)?,
                    Some((CatchVar::Register(i), _)) => self.write_u8(i)?,
                    _ => (),
                }
                self.inner.write_all(&action_buf)?;
            }
            Action::TypeOf => self.write_action_header(OpCode::TypeOf, 0)?,
            Action::WaitForFrame {
                frame,
                num_actions_to_skip,
            } => {
                self.write_action_header(OpCode::WaitForFrame, 3)?;
                self.write_u16(frame)?;
                self.write_u8(num_actions_to_skip)?;
            }
            Action::WaitForFrame2 {
                num_actions_to_skip,
            } => {
                self.write_action_header(OpCode::WaitForFrame2, 1)?;
                self.write_u8(num_actions_to_skip)?;
            }
            Action::With { ref actions } => {
                let mut action_buf = vec![];
                {
                    let mut fn_writer = Writer::new(&mut action_buf, self.version);
                    fn_writer.write_action_list(actions)?;
                }
                self.write_action_header(OpCode::With, action_buf.len())?;
                self.inner.write_all(&action_buf)?;
            }
            Action::Unknown { opcode, ref data } => {
                self.write_opcode_and_length(opcode, data.len())?;
                self.inner.write_all(data)?;
            }
        }

        Ok(())
    }

    pub fn write_action_header(&mut self, opcode: OpCode, length: usize) -> Result<()> {
        self.write_opcode_and_length(opcode as u8, length)
    }

    pub fn write_opcode_and_length(&mut self, opcode: u8, length: usize) -> Result<()> {
        self.write_u8(opcode)?;
        assert!(
            opcode >= 0x80 || length == 0,
            "Opcodes less than 0x80 must have length 0"
        );
        if opcode >= 0x80 {
            self.write_u16(length as u16)?;
        }
        Ok(())
    }

    fn write_push_value(&mut self, value: &Value) -> Result<()> {
        match *value {
            Value::Str(ref string) => {
                self.write_u8(0)?;
                self.write_c_string(string)?;
            }
            Value::Float(v) => {
                self.write_u8(1)?;
                self.write_f32(v)?;
            }
            Value::Null => {
                self.write_u8(2)?;
            }
            Value::Undefined => {
                self.write_u8(3)?;
            }
            Value::Register(v) => {
                self.write_u8(4)?;
                self.write_u8(v)?;
            }
            Value::Bool(v) => {
                self.write_u8(5)?;
                self.write_u8(v as u8)?;
            }
            Value::Double(v) => {
                self.write_u8(6)?;
                self.write_f64(v)?;
            }
            Value::Int(v) => {
                self.write_u8(7)?;
                self.write_i32(v)?;
            }
            Value::ConstantPool(v) => {
                if v < 256 {
                    self.write_u8(8)?;
                    self.write_u8(v as u8)?;
                } else {
                    self.write_u8(9)?;
                    self.write_u16(v)?;
                }
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn write_action() {
        for (swf_version, action, expected_bytes) in test_data::avm1_tests() {
            let mut written_bytes = Vec::new();
            Writer::new(&mut written_bytes, swf_version)
                .write_action(&action)
                .unwrap();
            if written_bytes != expected_bytes {
                panic!(
                    "Error writing action.\nTag:\n{:?}\n\nWrote:\n{:?}\n\nExpected:\n{:?}",
                    action, written_bytes, expected_bytes
                );
            }
        }
    }
}
