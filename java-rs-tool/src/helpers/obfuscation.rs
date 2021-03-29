use java_rs_pacific::{JavaClass, Error, ConstantPool, Constant, ToJavaUtf8Ext, Method};
use java_rs_pacific::attribute::{Attribute, Compatibility, Instruction};
use std::io::Cursor;

#[derive(Clone, Eq, PartialEq)]
pub enum Mode {
    Obfuscate,
    Deobfuscate
}

pub struct Options {
    pub mode: Mode,
    pub correct_strings: bool,
    pub remap_instructions: bool,
}

pub fn process_class(options: &Options, class: JavaClass) -> Result<JavaClass, Error> {
    if options.mode == Mode::Deobfuscate {
        let data = {
            let mut data = Vec::new();
            let mut writer = Cursor::new(&mut data);

            JavaClass {
                constant_pool: process_constant_pool(options, class.constant_pool)?,
                ..class
            }.write(&mut writer).expect("cannot write class file");

            data
        };

        let class = JavaClass::read(&mut Cursor::new(data)).expect("cannot read class file #2");

        Ok(JavaClass {
            methods: process_methods(options, &*class.methods)?.into(),
            ..class
        })
    } else {
        let data = {
            let mut data = Vec::new();
            let mut writer = Cursor::new(&mut data);

            JavaClass {
                methods: process_methods(options, &*class.methods)?.into(),
                ..class
            }.write(&mut writer).expect("cannot write class file");

            data
        };

        let class = JavaClass::read(&mut Cursor::new(data)).expect("cannot read class file #2");

        Ok(JavaClass {
            constant_pool: process_constant_pool(options, class.constant_pool)?,
            ..class
        })
    }
}

pub fn process_constant_pool(options: &Options, constant_pool: ConstantPool) -> Result<ConstantPool, Error> {
    if !options.correct_strings { return Ok(constant_pool) }
    let mut constants = Vec::new();

    for constant in constant_pool.0 {
        constants.push(process_constant(constant)?);
    }

    Ok(ConstantPool(constants))
}

pub fn process_constant(constant: Constant) -> Result<Constant, Error> {
    match &constant {
        Constant::Utf8(value) => Ok(process_constant_utf8(&value.to_java_utf8()?)?),
        Constant::InvalidUtf8(value) => Ok(process_constant_utf8(value)?),
        _ => Ok(constant),
    }
}

pub fn process_constant_utf8(input: &[u8]) -> Result<Constant, Error> {
    let mut result = Vec::new();

    for (index, byte) in input.iter().enumerate() {
        result.push(byte ^ input.len() as u8 ^ index as u8);
    }

    Ok(Constant::InvalidUtf8(result))
}

pub fn process_methods(options: &Options, methods: &[Method]) -> Result<Vec<Method>, Error> {
    let mut result = Vec::new();

    for method in methods {
        result.push(process_method(options, method)?);
    }

    Ok(result)
}

pub fn process_method(options: &Options, method: &Method) -> Result<Method, Error> {
    let mut attributes = Vec::new();

    for attribute in &*method.attributes {
        attributes.push(process_method_attribute(options, attribute)?);
    }

    Ok(Method {
        attributes: attributes.into(),
        ..*method
    })
}

pub fn process_method_attribute(options: &Options, attribute: &Attribute) -> Result<Attribute, Error> {
    let Options { mode, remap_instructions, .. } = options;

    match attribute {
        Attribute::Code {
            name,
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        } => {
            if !remap_instructions {
                return Ok(attribute.clone())
            }

            let instructions = match code {
                Compatibility::PreJava1(data) => data.to_vec(),
                Compatibility::Current(data) => data.to_vec(),
            };

            let mut result = Vec::new();

            for instruction in instructions {
                if mode == &Mode::Obfuscate {
                    result.push(match &instruction {
                        // Normal -> Fuck
                        Instruction::IAnd => Instruction::IShr,
                        Instruction::IShr => Instruction::IAnd,
                        Instruction::IAdd => Instruction::IOr,
                        Instruction::IOr => Instruction::IXor,
                        Instruction::IXor => Instruction::IAdd,
                        Instruction::IShl => Instruction::IUShr,
                        Instruction::IUShr => Instruction::IRem,
                        Instruction::IRem => Instruction::IShl,
                        Instruction::IDiv => Instruction::INeg,
                        Instruction::INeg => Instruction::IDiv,
                        _ => instruction,
                    })
                } else {
                    result.push(match &instruction {
                        // Fuck -> Normal
                        Instruction::IShr => Instruction::IAnd,
                        Instruction::IAnd => Instruction::IShr,
                        Instruction::IOr => Instruction::IAdd,
                        Instruction::IXor => Instruction::IOr,
                        Instruction::IAdd => Instruction::IXor,
                        Instruction::IUShr => Instruction::IShl,
                        Instruction::IRem => Instruction::IUShr,
                        Instruction::IShl => Instruction::IRem,
                        Instruction::INeg => Instruction::IDiv,
                        Instruction::IDiv => Instruction::INeg,
                        _ => instruction,
                    })
                }
            }

            match code {
                Compatibility::PreJava1(_) => Ok(Attribute::Code {
                    name: *name,
                    max_stack: max_stack.clone(),
                    max_locals: max_locals.clone(),
                    code: Compatibility::PreJava1(result.into()),
                    exception_table: exception_table.to_vec().into(),
                    attributes: attributes.to_vec().into(),
                }),
                Compatibility::Current(_) => Ok(Attribute::Code {
                    name: *name,
                    max_stack: max_stack.clone(),
                    max_locals: max_locals.clone(),
                    code: Compatibility::Current(result.into()),
                    exception_table: exception_table.to_vec().into(),
                    attributes: attributes.to_vec().into(),
                }),
            }
        }
        _ => Ok(attribute.clone()),
    }
}
