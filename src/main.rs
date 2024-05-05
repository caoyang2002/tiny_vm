use std::collections::BTreeMap;
use std::io::Read;

// Pointers are just indices into a Vec
// 指针知识 Vec 中的索引
type Pointer = usize;

// The Program is a list of instructions
// Program 是指令列表
type Program<'a> = &'a [Instruction];

// A Label is a name and an instruction pointer
// 标签是名字和指令指针
type Label<'a> = (&'a str, Pointer);
type Labels<'a> = BTreeMap<&'a str, Pointer>;

// A procedure has a name, a start instruction pointer,
// and an end instruction pointer.
// The ending instruction pointer is just used to skip over
// the procedure.
// 过程（procedure）有一个名称、起始指令指针和结束指令指针。
// 结束指令指针仅用于跳过过程。
type Procedures<'a> = BTreeMap<&'a str, (Pointer, Pointer)>;

// A StackFrame has an offset and an instruction pointer
// to return to.
// The offset is used for the Get/Set and GetArg/SetArg instructions.
//
// Example:
// With stack [1, 3, 2] on instruction 24, call a procedure
// at location 96.
//
// The stack frame to be pushed will look like:
// StackFrame {
//      stack_offset: 3, // the length of the stack before calling the procedure
//      ip: i,
// }
//
// The current instruction pointer will be set to 96.
//
// Inside the procedure, let's say a few values are pushed,
// resulting in a stack [1, 3, 2, 5, 7, 4].
//
// Stack values are now accessed by Get/Set and GetArg/SetArg
// relatively to the stack offset (denoted by the pipe '|'):
//
//       GetArg 2    GetArg 1    GetArg 0     Get 0       Get 1       Get 2
// [        1,          3,          2,    |     5,          7,          4       ]
// ----------------
// 栈帧（StackFrame）有一个偏移量和一个要返回的指令指针。
// 偏移量用于Get/Set和GetArg/SetArg指令。
// 例如：
// 在执行指令24时，栈上有一个[1, 3, 2]，调用位于96位置的过程。
// 要推入的栈帧看起来像这样：
// StackFrame {
//      stack_offset: 3, // 调用过程前栈的长度
//      ip: i,
// }
// 当前指令指针将被设置为96。
// 在过程内部，假设推入了一些值，导致栈变为[1, 3, 2, 5, 7, 4]。
// 现在通过Get/Set和GetArg/SetArg相对栈偏移量（用管道符号 '|' 表示）访问栈值：
//       GetArg 2    GetArg 1    GetArg 0     Get 0       Get 1       Get 2
// [        1,          3,          2,    |     5,          7,          4       ]
struct StackFrame {
    pub stack_offset: Pointer,
    pub ip: Pointer,
}

// The CallStack is just a Vec of StackFrames.
// 调用栈（CallStack）只是栈帧（StackFrame）的Vec。
type CallStack = Vec<StackFrame>;

// Since the only values allowed by this VM are isizes,
// the Stack is just a Vec of isizes.
//
// I made a wrapper type just to panic and crash the program
// on any errors. In a real VM you'd want to add proper error
// handling.
// ---------------
// 由于这个虚拟机只允许 isize 类型的值，因此栈（Stack）只是isize的Vec。
// 我创建了一个包装类型，只是为了在任何错误发生时让程序panic和崩溃。
// 在一个真正的虚拟机中，你会想要添加适当的错误处理。
struct Stack(Vec<isize>);

impl Stack {
    fn push(&mut self, v: isize) {
        self.0.push(v);
    }

    fn pop(&mut self) -> isize {
        self.0.pop().expect("popped an empty stack")
    }

    fn peek(&mut self) -> isize {
        *self.0.last().expect("peeked an empty stack")
    }

    fn peek_mut(&mut self) -> &mut isize {
        self.0.last_mut().expect("peeked an empty stack")
    }

    fn get(&self, i: usize) -> &isize {
        self.0.get(i).expect("accessed a nonexistent stack index")
    }

    fn get_mut(&mut self, i: usize) -> &mut isize {
        self.0
            .get_mut(i)
            .expect("mutably accessed a nonexistent stack index")
    }
}

// For simplicity, this VM runs off of tagged union
// Instructions which carry data with them. For that reason,
// this isn't strictly a *bytecode* interpreter, since instructions
// take 16 bytes.
//
// Usually, you would want to store each Instruction
// as just a discriminant (e.g. Push or Jump) so that
// they fit in one byte each.
//
// The instruction arguments would then be read byte by byte from
// the code.
//
// An explanation of the individual instructions is in the
// `interpret()` function.
// -------------------
// 为了简单起见，这个虚拟机运行在带有数据的标记联合指令上。
// 因此，这并不是一个严格的*字节码*解释器，因为指令占用16个字节。
// 通常，你希望将每个指令仅存储为一个枚举项（例如Push或Jump），以便它们每个都适合一个字节。
// 然后，指令参数将逐字节从代码中读取。
// 对各个指令的解释在`interpret()`函数中有说明。
#[derive(Debug)]
enum Instruction {
    Push(isize),
    Pop,
    Add,
    Sub,
    Incr,
    Decr,
    Mul,
    Div,
    Jump(Pointer),
    JE(Pointer),
    JNE(Pointer),
    JGT(Pointer),
    JLT(Pointer),
    JGE(Pointer),
    JLE(Pointer),
    Get(Pointer),
    Set(Pointer),
    GetArg(Pointer),
    SetArg(Pointer),
    Noop,
    Print,
    PrintC,
    PrintStack,
    Call(Pointer),
    Ret,
}

fn interpret<'a>(program: Program<'a>) {
    use Instruction::*;

    let mut stack: Stack = Stack(Vec::new());
    let mut pointer: Pointer = 0;
    let mut call_stack = CallStack::new();

    while let Some(instruction) = program.get(pointer) {
        pointer += 1;

        match instruction {
            // Noop doesn't do anything. However, it's used as a placeholder
            // for labels and procedures in the code.
            // Noop不执行任何操作。然而，它被用作代码中的标签和过程的占位符
            Noop => {}

            // Push pushes a value to the top of the stack.
            // Push将一个值推到栈顶。
            Push(d) => stack.push(*d),

            // Pop removes a value from the top of the stack.
            // Pop从栈顶移除一个值。
            Pop => {
                stack.pop();
            }

            // Add pops the two top values, adds them, and pushes
            // the result.
            //
            // Before:
            // [.., b, a]
            //
            // After:
            // [.., a + b]
            // ---------------
            // Add弹出栈顶的两个值，将它们相加，并将结果推回栈顶。
            // 之前：
            // [.., b, a]
            // 之后：
            // [.., a + b]
            Add => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a + b)
            }

            // Sub pops the two top values, and pushes the difference.
            // Importantly, the order of operations is switched.
            //
            // This is a bit more intuitive because the stack is
            // usually reasoned about from left to right.
            //
            // Before:
            // [.., b, a]
            //
            // After:
            // [.., b - a]
            // -------------
            // Sub弹出栈顶的两个值，并推回它们的差。
            // 重要的是，操作顺序被交换了。
            //
            // 这是因为栈通常从左到右推理。
            //
            // 之前：
            // [.., b, a]
            //
            // 之后：
            // [.., b - a]
            Sub => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b - a)
            }

            // I think you can figure out Mul and Div
            // 我认为你能理解Mul和Div
            Mul => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(a * b) 
            }
            Div => {
                let (a, b) = (stack.pop(), stack.pop());
                stack.push(b / a)
            }

            // Incr and Decr increment or decrement the value
            // at the top of the stack.
            //
            // These instructions are redundant because of Add and Sub,
            // but they improve performance significantly because they
            // remove an unecessary Push.
            // ----------------
            // Incr和Decr增加或减少栈顶的值。
            // 这些指令是多余的，因为Add和Sub也能做到，
            // 但是它们显著提高了性能，因为它们去除了不必要的Push。
            Incr => *stack.peek_mut() += 1,
            Decr => *stack.peek_mut() -= 1,

            // Jump unconditionally changes the stack pointer
            // Jump无条件地改变栈指针
            Jump(p) => pointer = *p,

            // JE changes the stack pointer if the value
            // on top of the stack is zero. This is generally
            // used after Sub for equality testing, hence the
            // name of the instruction, Jump (if) Equal.
            //
            // Example:
            //
            // PrintStack -- [.., b, a]
            // Sub
            // PrintStack -- [.., b - a] // this will be zero if a and b are equal
            // JE i // jumps to Instruction i if a and b were equal.
            // ---------------
            // JE如果栈顶的值为零，则改变栈指针。这通常在Sub之后用于等式测试，因此得名Jump (if) Equal。
            //
            // 示例：
            //
            // PrintStack -- [.., b, a]
            // Sub
            // PrintStack -- [.., b - a] // 如果a和b相等，这将为零
            // JE i // 如果a和b相等，则跳转到指令i。
            JE(p) => {
                if stack.peek() == 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // JNE (Jump Not Equal) changes the stack pointer
            // if the value on top of the stack is *not* zero.
            // JNE (Jump Not Equal)如果栈顶的值*不是*零，则改变栈指针。
            JNE(p) => {
                if stack.peek() != 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // JGT (Jump Greater Than) changes the stack pointer
            // if the value on top of the stack is greater than zero.
            // JGT (Jump Greater Than)如果栈顶的值大于零，则改变栈指针。
            JGT(p) => {
                if stack.peek() > 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // JLT (Jump Less Than) changes the stack pointer
            // if the value on top of the stack is less than zero.
            // JLT (Jump Less Than)如果栈顶的值小于零，则改变栈指针。
            JLT(p) => {
                if stack.peek() < 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // JGE (Jump Greater Equal) changes the stack pointer
            // if the value on top of the stack is greater than
            // or equal to zero.
            // JGE (Jump Greater Equal)如果栈顶的值大于或等于零，则改变栈指
            JGE(p) => {
                if stack.peek() >= 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // JLE (Jump Less Equal) changes the stack pointer
            // if the value on top of the stack is greater than
            // or equal to zero.
            // JLE (Jump Less Equal)如果栈顶的值小于或等于零，则改变栈指针。
            JLE(p) => {
                if stack.peek() <= 0 {
                    stack.pop();
                    pointer = *p;
                }
            }

            // The above instructions can be confusing because they
            // don't quite match the naming semantics of JE and JNE.
            // For example, when used after a Sub, JGT will jump if
            // a was *less* than b, not greater than it, because
            // if a is *less* than b, b - a will be *greater* than zero.

            // Get pushes the value at index i to the top of the stack.
            //
            // Example:
            //
            // PrintStack -- [0, 1, 3, 2, 5]
            // Get 2
            // PrintStack -- [0, 1, 3, 2, 5, 3]
            //
            // Remember that values are indexed relatively to the stackframe
            // as explained near the start of the file.
            // ----------------------
            // 上面的指令可能会让人困惑，因为它们与JE和JNE的命名语义不完全匹配。
            // 例如，在Sub之后使用JGT将跳转，如果a比b*小*，而不是更大，因为如果a比b小，b - a将*大于*零。
            // Get将索引i的值推到栈顶。
            //
            // 示例：
            //
            // PrintStack -- [0, 1, 3, 2, 5]
            // Get 2
            // PrintStack -- [0, 1, 3, 2, 5, 3]
            //
            // 记住，值是相对于栈帧索引的，如文件开头所述。
            Get(i) => stack.push(*stack.get(*i + call_stack.last().map_or(0, |s| s.stack_offset))),

            // Set sets the value at index i to be equal to the value
            // at the top of the stack. It does *not* pop the top value.
            //
            // Example:
            //
            // PrintStack -- [0, 1, 3, 2, 5]
            // Set 2
            // PrintStack -- [0, 1, 5, 2, 5]
            //
            // Remember that values are indexed relatively to the stackframe
            // as explained near the start of the file.
            // ---------------
            // Set指令将栈顶的值设置为索引i处的值，但不弹出栈顶的值。
            //
            // 示例：
            //
            // PrintStack -- [0, 1, 3, 2, 5]
            // Set 2
            // PrintStack -- [0, 1, 5, 2, 5]
            //
            // 记住，值是相对于栈帧索引的，如文件开头所述。
            Set(i) => {
                *stack
                    .0
                    .get_mut(*i + call_stack.last().map_or(0, |s| s.stack_offset))
                    .unwrap() = stack.peek()
            }

            // GetArg and SetArg mirror Get and Set.
            // GetArg和SetArg是Get和Set的镜像。
            GetArg(i) => stack.push(
                *stack
                    .0
                    .get(call_stack.last().unwrap().stack_offset - 1 - *i)
                    .unwrap(),
            ),
            SetArg(i) => {
                let offset_i = call_stack.last().unwrap().stack_offset - 1 - *i;
                let new_val = stack.peek();
                *stack.get_mut(offset_i) = new_val;
            }

            // Print prints the value at the top of the stack.
            // Print打印栈顶的值。
            Print => print!("{}", stack.peek()),

            // PrintC prints the value at the top of the stack
            // as an ASCII character.
            // PrintC将栈顶的值作为ASCII字符打印出来。
            PrintC => print!("{}", stack.peek() as u8 as char),

            // PrintStack prints the whole stack. It's meant to be
            // used for debugging.
            // PrintStack打印整个栈。它主要用于调试。
            PrintStack => println!("{:?}", stack.0),

            // Call calls a procedure, pushing a new StackFrame.
            // Details about the StackFrame can be found near the
            // start of the file.
            // Call调用一个过程，并推入一个新的栈帧。
            // 栈帧的详细信息可以在文件的开头找到。
            Call(p) => {
                call_stack.push(StackFrame {
                    stack_offset: stack.0.len(),
                    ip: pointer,
                });
                pointer = *p;
            }

            // Ret returns from the current procedure, popping the
            // stack frame from the top of the call stack and returning
            // to the instruction list at the index right after it was called at.
            // Ret从当前过程返回，从调用栈顶部弹出栈帧，并返回到调用它的指令列表的下一个索引处。
            Ret => pointer = call_stack.pop().unwrap().ip,
        }
    }
}

// The next instructions aren't really part of the VM, they are essentially
// an extremely simplistic compiler. That's because the VM doesn't quite
// support labels. It doesn't support named procedures either; it specifies
// for procedures that contain an index into the Instruction list. Additionally,
// this simplistic compiler allows for some basic comments.

// `parse_instruction` takes a line split by spaces and returns a singular
// instruction that the line represents.
//
// Labels must be obtained by preprocessing the string, it's just a map of
// names to their index in the instruction set.
//
// Procedures is similar, but it contains the instruction to jump to when called
// along with the instruction to jump to in order to skip the Procedure declaration.
// Procedures are encoded directly into the list of instructions, so the actual
// Procedure declaration is replaced by a Jump instruction to skip over it.
//
// Example:
//
// Procedure proc_name -- Line n
// ...
// End -- line e
//
// Gets turned into
//
// Jump e -- Line n
// ...
// Noop -- line e
// -------------
// 下面的指令实际上并不是虚拟机的一部分，它们本质上是一个非常简单的编译器。
// 这是因为虚拟机并不完全支持标签。它也不支持命名过程；它为过程指定了一个指向指令列表中索引的值。
// 此外，这个简单的编译器允许一些基本的注释。

// `parse_instruction`接受由空格分隔的一行，并返回该行所代表的单一指令。
//
// 必须通过预处理字符串来获取标签，它仅仅是将名称映射到指令集中的索引。
//
// 过程类似，但它包含了当被调用时跳转的指令，以及为了跳过过程声明而跳转的指令。
// 过程直接编码到指令列表中，因此实际的过程声明被一个跳转指令替换，以跳过它。
//
// 示例：
//
// Procedure proc_name -- 第n行
// ...
// End -- 第e行
//
// 被转换成
//
// Jump e -- 第n行
// ...
// Noop -- 第e行
fn parse_instruction(s: &[&str], labels: &Labels, procedures: &Procedures) -> Instruction {
    use Instruction::*;

    match s {
        ["Push", x] => Push(x.parse::<isize>().unwrap()),
        ["Pop"] => Pop,
        ["Add"] => Add,
        ["Sub"] => Sub,
        ["Mul"] => Mul,
        ["Div"] => Div,
        ["Incr"] => Incr,
        ["Decr"] => Decr,
        ["Jump", l] => Jump(*labels.get(l).unwrap()),
        ["JE", l] => JE(*labels.get(l).unwrap()),
        ["JNE", l] => JNE(*labels.get(l).unwrap()),
        ["JGE", l] => JGE(*labels.get(l).unwrap()),
        ["JLE", l] => JLE(*labels.get(l).unwrap()),
        ["JGT", l] => JGT(*labels.get(l).unwrap()),
        ["JLT", l] => JLT(*labels.get(l).unwrap()),
        ["Get", p] => Get(p.parse::<Pointer>().unwrap()),
        ["Set", p] => Set(p.parse::<Pointer>().unwrap()),
        ["GetArg", p] => GetArg(p.parse::<Pointer>().unwrap()),
        ["SetArg", p] => SetArg(p.parse::<Pointer>().unwrap()),
        ["Print"] => Print,
        ["PrintC"] => PrintC,
        ["PrintStack"] => PrintStack,
        ["Proc", proc] => Jump(procedures.get(proc).unwrap().1),
        ["Call", proc] => Call(procedures.get(proc).unwrap().0 + 1),
        ["Ret"] => Ret,
        ["label", ..] | ["End"] => Noop,
        l => panic!("Invalid instruction: {:?}", l),
    }
}

// find_label takes a line split by spaces and the label it represents,
// or None if it does not represent a label.
// find_label接受由空格分隔的一行和它所代表的标签，或者如果它不代表任何标签，则返回None。
fn find_label<'a>(i: Pointer, s: &'a [&'a str]) -> Option<Label> {
    if let ["label", l] = s {
        Some((l, i))
    } else {
        None
    }
}

// find_procedures takes a list of lines split on space and
// returns the procedures declared.
// find_procedures接受一个由空格分隔的行列表，并返回声明的过程。
fn find_procedures<'a>(lines: &'a [Vec<&str>]) -> Procedures<'a> {
    let mut ip = 0;
    let mut res = Procedures::new();

    while ip < lines.len() {
        if let ["Proc", proc_name] = lines[ip].as_slice() {
            let start_ip = ip;
            while lines[ip] != &["End"] {
                ip += 1;
            }
            res.insert(proc_name, (start_ip, ip + 1));
        } else {
            ip += 1;
        }
    }

    res
}

// main 函数的开始，它返回 std::io::Result 类型，用于处理可能的 I/O 错误。
fn main() -> std::io::Result<()> {
    // 收集命令行参数到 args 向量中
    let args: Vec<String> = std::env::args().collect();
    // 尝试打开命令行参数中的第一个文件（通常是程序的路径后的文件）
    let mut f = std::fs::File::open(&args[1])?;

    // 创建一个 String 来存储文件内容
    let mut buffer = String::new();
    // 读取文件内容到 buffer 中，如果出错则返回错误
    f.read_to_string(&mut buffer)?;

    // 将 buffer 中的文本按行分割，去除空行和注释行（以"--"开头的行），并收集到 line_splits 向量中。
    let line_splits = buffer
        .split('\n')
        // 对原始迭代器中的每个字符串切片 s 应用 split_whitespace() 方法来分割字符串，
        // 然后使用 collect::<Vec<_>>() 将分割后的单词收集到一个 Vec（向量）中。
        // 最终结果是一个新的迭代器，迭代器中的每个元素都是一个包含单词的 Vec。
        .map(|s| s.split_whitespace().collect::<Vec<_>>())
        // 对迭代器中的每个元素 s 进行判断，
        // 如果 s 不是一个空行且不是以 "--" 开头的注释行，则保留该元素；否则，过滤掉该元素。
        .filter(|s| !matches!(s.as_slice(), [] | ["--", ..]))
        .collect::<Vec<_>>();

    // 从 line_splits 中提取标签信息，存储到 labels 集合中。
    let labels: Labels = line_splits
        .iter()
        // 为每个行分割后的向量提供一个索引。
        .enumerate()
        // 寻找标签并将其转换为 Label 类型。
        .filter_map(|(i, s)| find_label(i, s.as_slice()))
        .collect();

    // 从 line_splits 中提取过程（procedures）信息，存储到 procedures 集合中。
    let procedures: Procedures = find_procedures(line_splits.as_slice());

    // 解析 line_splits 中的每一行，将其转换为 Instruction 枚举类型，并存储到 instructions 向量中。
    let instructions: Vec<Instruction> = line_splits
        .iter()
        // 对每一行进行解析。
        .map(|s| parse_instruction(s.as_slice(), &labels, &procedures))
        .collect();

    // 解释执行 instructions 向量中的指令。
    interpret(&instructions[..]);

    // 如果程序成功执行到这里，返回 Ok(()) 表示成功。
    Ok(())
}
