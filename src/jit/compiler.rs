// JIT compiler expression compilation methods.
//
// Contains compile_expr, compile_expr_with_vars, and compile_expr_with_context
// which translate AST nodes to Cranelift IR.
//
// Extracted from jit/mod.rs for file-health compliance (<2000 lines).

use cranelift::codegen::ir::BlockArg;
use cranelift::prelude::*;
use std::collections::HashMap;

use crate::interpreter::parser::{AstNode, BinaryOperator, Parser, Pattern, UnaryOperator};
use super::{JitCompiler, JitError, StringContext};

impl JitCompiler {
    /// Compile AST expression to Cranelift IR value (no variables)
    pub(crate) fn compile_expr(
        ast: &AstNode,
        builder: &mut FunctionBuilder,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let mut var_counter = 0;

        Self::compile_expr_with_context(
            ast,
            builder,
            &HashMap::new(),
            &mut HashMap::new(),
            &mut var_counter,
            compiled_functions,
            string_ctx,
            struct_defs,
        )
    }

    /// Compile AST expression to Cranelift IR value (with variable context)
    pub(crate) fn compile_expr_with_vars(
        ast: &AstNode,
        builder: &mut FunctionBuilder,
        variables: &HashMap<String, Value>,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let mut var_counter = 0;

        Self::compile_expr_with_context(
            ast,
            builder,
            variables,
            &mut HashMap::new(),
            &mut var_counter,
            compiled_functions,
            string_ctx,
            struct_defs,
        )
    }

    /// Compile AST expression to Cranelift IR value (with full context)
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::only_used_in_recursion)]
    pub(crate) fn compile_expr_with_context(
        ast: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        match ast {
            AstNode::Return { value } => {
                Self::compile_return(value, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::Block { statements } => {
                Self::compile_block(statements, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::LetDecl { name, value } => {
                Self::compile_let_decl(name, value, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::Assignment { name, value } => {
                Self::compile_assignment(name, value, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::IntegerLiteral(n) => Ok(builder.ins().iconst(types::I64, *n)),
            AstNode::BooleanLiteral(b) => Ok(builder.ins().iconst(types::I64, if *b { 1 } else { 0 })),
            AstNode::StringLiteral(s) => {
                let ptr = string_ctx.intern_string(s);
                Ok(builder.ins().iconst(types::I64, ptr))
            }
            AstNode::FString { content } => {
                Self::compile_fstring(content, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::PathExpr { segments } => {
                let _qualified_name = segments.join("::");
                Ok(builder.ins().iconst(types::I64, 0))
            }
            AstNode::FloatLiteral(f) => Ok(builder.ins().f64const(*f)),
            AstNode::Identifier(name) => Self::compile_identifier(name, builder, parameters, local_vars),
            AstNode::BinaryOp { left, op, right } => {
                Self::compile_binary_op(left, op, right, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::UnaryOp { op, operand } => {
                Self::compile_unary_op(op, operand, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::WhileLoop { condition, body } => {
                Self::compile_while_loop(condition, body, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::ForLoop { var, iterable, body } => {
                Self::compile_for_loop(var, iterable, body, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::IfExpr { condition, then_branch, else_branch } => {
                Self::compile_if_expr(condition, then_branch, else_branch, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::FunctionCall { name, args } => {
                Self::compile_function_call(name, args, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::VectorLiteral { elements } => {
                Self::compile_vector_literal(elements, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::TupleLiteral { elements } => {
                Self::compile_tuple_literal(elements, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::IndexAccess { expr, index } => {
                Self::compile_index_access(expr, index, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::CompoundAssignment { lhs, op, rhs } => {
                Self::compile_compound_assignment(lhs, op, rhs, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::FieldAccess { expr, field } => {
                Self::compile_field_access(expr, field, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::StructDef { name, fields } => {
                let field_names: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();
                struct_defs.insert(name.clone(), field_names);
                Ok(builder.ins().iconst(types::I64, 0))
            }
            AstNode::StructLiteral { name, fields } => {
                Self::compile_struct_literal(name, fields, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::HashMapLiteral { pairs } => {
                Self::compile_hashmap_literal(pairs, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::MatchExpr { expr, arms } => {
                Self::compile_match_expr(expr, arms, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::MethodCall { receiver, method, args } => {
                Self::compile_method_call(receiver, method, args, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::TupleDestruct { names, value } => {
                Self::compile_tuple_destruct(names, value, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::TypeCast { expr, target_type } => {
                Self::compile_type_cast(expr, target_type, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::VecMacro { elements, repeat_count } => {
                Self::compile_vec_macro(elements, repeat_count, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            _ => Err(JitError::UnsupportedNode(format!(
                "Cannot compile AST node: {:?}",
                ast
            ))),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_return(
        value: &Option<Box<AstNode>>,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let dummy = builder.ins().iconst(types::I64, 0);

        if let Some(expr) = value {
            let return_value = Self::compile_expr_with_context(
                expr, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;

            let return_value = if builder.func.dfg.value_type(return_value) == types::F64 {
                builder.ins().bitcast(types::I64, MemFlags::new(), return_value)
            } else {
                return_value
            };

            builder.ins().return_(&[return_value]);
        } else {
            let zero = builder.ins().iconst(types::I64, 0);
            builder.ins().return_(&[zero]);
        }
        Ok(dummy)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_block(
        statements: &[AstNode],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let mut result = builder.ins().iconst(types::I64, 0);
        for stmt in statements {
            result = Self::compile_expr_with_context(
                stmt, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;
        }
        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_let_decl(
        name: &str,
        value: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let init_value = Self::compile_expr_with_context(
            value, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let value_type = builder.func.dfg.value_type(init_value);
        let var = builder.declare_var(value_type);
        builder.def_var(var, init_value);
        local_vars.insert(name.to_string(), var);

        Ok(builder.ins().iconst(types::I64, 0))
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_assignment(
        name: &str,
        value: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let var = *local_vars.get(name).ok_or_else(|| {
            JitError::UnsupportedNode(format!("Undefined variable: {}", name))
        })?;

        let new_value = Self::compile_expr_with_context(
            value, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        builder.def_var(var, new_value);
        Ok(builder.ins().iconst(types::I64, 0))
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_fstring(
        content: &str,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let mut result = String::new();
        let mut chars = content.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '{' {
                let mut expr_str = String::new();
                let mut depth = 1;
                for ch in chars.by_ref() {
                    if ch == '{' {
                        depth += 1;
                        expr_str.push(ch);
                    } else if ch == '}' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                        expr_str.push(ch);
                    } else {
                        expr_str.push(ch);
                    }
                }

                let mut parser = Parser::new(&expr_str);
                let expr_ast_vec = parser.parse().map_err(|e| {
                    JitError::UnsupportedNode(format!(
                        "Failed to parse f-string expression '{}': {:?}",
                        expr_str, e
                    ))
                })?;

                let expr_ast = expr_ast_vec.nodes().first().ok_or_else(|| {
                    JitError::UnsupportedNode(format!(
                        "Empty AST from f-string expression '{}'",
                        expr_str
                    ))
                })?;

                let _expr_value = Self::compile_expr_with_context(
                    expr_ast, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
                )?;

                result.push_str("<value>");
            } else {
                result.push(ch);
            }
        }

        let ptr = string_ctx.intern_string(&result);
        let val = builder.ins().iconst(types::I64, ptr);
        Ok(val)
    }

    fn compile_identifier(
        name: &str,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
    ) -> Result<Value, JitError> {
        if let Some(&var) = local_vars.get(name) {
            let value = builder.use_var(var);
            Ok(value)
        } else if let Some(&value) = parameters.get(name) {
            Ok(value)
        } else {
            Err(JitError::UnsupportedNode(format!(
                "Undefined variable: {}",
                name
            )))
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_binary_op(
        left: &AstNode,
        op: &BinaryOperator,
        right: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let lhs = Self::compile_expr_with_context(
            left, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;
        let rhs = Self::compile_expr_with_context(
            right, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let lhs_type = builder.func.dfg.value_type(lhs);
        let rhs_type = builder.func.dfg.value_type(rhs);
        let is_float = lhs_type == types::F64 || rhs_type == types::F64;

        let result = if is_float {
            Self::compile_float_binary_op(op, lhs, rhs, builder)?
        } else {
            Self::compile_int_binary_op(op, lhs, rhs, builder)
        };

        Ok(result)
    }

    fn compile_float_binary_op(
        op: &BinaryOperator,
        lhs: Value,
        rhs: Value,
        builder: &mut FunctionBuilder,
    ) -> Result<Value, JitError> {
        match op {
            BinaryOperator::Add => Ok(builder.ins().fadd(lhs, rhs)),
            BinaryOperator::Subtract => Ok(builder.ins().fsub(lhs, rhs)),
            BinaryOperator::Multiply => Ok(builder.ins().fmul(lhs, rhs)),
            BinaryOperator::Divide => Ok(builder.ins().fdiv(lhs, rhs)),
            BinaryOperator::Equal => {
                let cmp = builder.ins().fcmp(FloatCC::Equal, lhs, rhs);
                Ok(builder.ins().uextend(types::I64, cmp))
            }
            BinaryOperator::NotEqual => {
                let cmp = builder.ins().fcmp(FloatCC::NotEqual, lhs, rhs);
                Ok(builder.ins().uextend(types::I64, cmp))
            }
            BinaryOperator::LessThan => {
                let cmp = builder.ins().fcmp(FloatCC::LessThan, lhs, rhs);
                Ok(builder.ins().uextend(types::I64, cmp))
            }
            BinaryOperator::GreaterThan => {
                let cmp = builder.ins().fcmp(FloatCC::GreaterThan, lhs, rhs);
                Ok(builder.ins().uextend(types::I64, cmp))
            }
            BinaryOperator::LessEqual => {
                let cmp = builder.ins().fcmp(FloatCC::LessThanOrEqual, lhs, rhs);
                Ok(builder.ins().uextend(types::I64, cmp))
            }
            BinaryOperator::GreaterEqual => {
                let cmp = builder.ins().fcmp(FloatCC::GreaterThanOrEqual, lhs, rhs);
                Ok(builder.ins().uextend(types::I64, cmp))
            }
            BinaryOperator::Modulo => {
                Err(JitError::UnsupportedNode(
                    "Modulo operator not supported for floats".to_string(),
                ))
            }
            BinaryOperator::And | BinaryOperator::Or => {
                Err(JitError::UnsupportedNode(
                    "Logical operators not supported for floats".to_string(),
                ))
            }
        }
    }

    fn compile_int_binary_op(
        op: &BinaryOperator,
        lhs: Value,
        rhs: Value,
        builder: &mut FunctionBuilder,
    ) -> Value {
        match op {
            BinaryOperator::Add => builder.ins().iadd(lhs, rhs),
            BinaryOperator::Subtract => builder.ins().isub(lhs, rhs),
            BinaryOperator::Multiply => builder.ins().imul(lhs, rhs),
            BinaryOperator::Divide => builder.ins().sdiv(lhs, rhs),
            BinaryOperator::Modulo => builder.ins().srem(lhs, rhs),
            BinaryOperator::Equal => {
                let cmp = builder.ins().icmp(IntCC::Equal, lhs, rhs);
                builder.ins().uextend(types::I64, cmp)
            }
            BinaryOperator::NotEqual => {
                let cmp = builder.ins().icmp(IntCC::NotEqual, lhs, rhs);
                builder.ins().uextend(types::I64, cmp)
            }
            BinaryOperator::LessThan => {
                let cmp = builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs);
                builder.ins().uextend(types::I64, cmp)
            }
            BinaryOperator::GreaterThan => {
                let cmp = builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs);
                builder.ins().uextend(types::I64, cmp)
            }
            BinaryOperator::LessEqual => {
                let cmp = builder.ins().icmp(IntCC::SignedLessThanOrEqual, lhs, rhs);
                builder.ins().uextend(types::I64, cmp)
            }
            BinaryOperator::GreaterEqual => {
                let cmp = builder
                    .ins()
                    .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs);
                builder.ins().uextend(types::I64, cmp)
            }
            BinaryOperator::And => {
                let zero = builder.ins().iconst(types::I64, 0);
                let lhs_bool = builder.ins().icmp(IntCC::NotEqual, lhs, zero);
                let rhs_bool = builder.ins().icmp(IntCC::NotEqual, rhs, zero);
                let and = builder.ins().band(lhs_bool, rhs_bool);
                builder.ins().uextend(types::I64, and)
            }
            BinaryOperator::Or => {
                let zero = builder.ins().iconst(types::I64, 0);
                let lhs_bool = builder.ins().icmp(IntCC::NotEqual, lhs, zero);
                let rhs_bool = builder.ins().icmp(IntCC::NotEqual, rhs, zero);
                let or = builder.ins().bor(lhs_bool, rhs_bool);
                builder.ins().uextend(types::I64, or)
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_unary_op(
        op: &UnaryOperator,
        operand: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let value = Self::compile_expr_with_context(
            operand, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let result = match op {
            UnaryOperator::Negate => {
                let value_type = builder.func.dfg.value_type(value);
                if value_type == types::F64 {
                    builder.ins().fneg(value)
                } else {
                    builder.ins().ineg(value)
                }
            }
            UnaryOperator::Not => {
                let zero = builder.ins().iconst(types::I64, 0);
                let is_zero = builder.ins().icmp(IntCC::Equal, value, zero);
                builder.ins().uextend(types::I64, is_zero)
            }
            UnaryOperator::Plus => value,
            UnaryOperator::Dereference => return Err(JitError::UnsupportedNode(
                "Dereference operator not supported in JIT (requires pointer types)"
                    .to_string(),
            )),
        };

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_while_loop(
        condition: &AstNode,
        body: &[AstNode],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let loop_header = builder.create_block();
        let loop_body = builder.create_block();
        let loop_exit = builder.create_block();

        builder.ins().jump(loop_header, &[]);

        builder.switch_to_block(loop_header);

        let cond_value = Self::compile_expr_with_context(
            condition, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let zero = builder.ins().iconst(types::I64, 0);
        let is_true = builder.ins().icmp(IntCC::NotEqual, cond_value, zero);

        builder.ins().brif(is_true, loop_body, &[], loop_exit, &[]);

        builder.switch_to_block(loop_body);
        builder.seal_block(loop_body);

        for stmt in body {
            Self::compile_expr_with_context(
                stmt, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;
        }

        builder.ins().jump(loop_header, &[]);
        builder.seal_block(loop_header);

        builder.switch_to_block(loop_exit);
        builder.seal_block(loop_exit);

        let result = builder.ins().iconst(types::I64, 0);
        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_for_loop(
        var: &str,
        iterable: &AstNode,
        body: &[AstNode],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let (start_expr, end_expr) = match iterable {
            AstNode::Range { start, end } => (start, end),
            _ => {
                return Err(JitError::UnsupportedNode(
                    "For loop iterable must be a Range".to_string(),
                ))
            }
        };

        let start_value = Self::compile_expr_with_context(
            start_expr, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let loop_var = builder.declare_var(types::I64);
        builder.def_var(loop_var, start_value);
        local_vars.insert(var.to_string(), loop_var);

        let loop_header = builder.create_block();
        let loop_body = builder.create_block();
        let loop_exit = builder.create_block();

        builder.ins().jump(loop_header, &[]);

        builder.switch_to_block(loop_header);

        let current_var = builder.use_var(loop_var);

        let end_value = Self::compile_expr_with_context(
            end_expr, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let condition = builder
            .ins()
            .icmp(IntCC::SignedLessThan, current_var, end_value);

        builder
            .ins()
            .brif(condition, loop_body, &[], loop_exit, &[]);

        builder.switch_to_block(loop_body);
        builder.seal_block(loop_body);

        for stmt in body {
            Self::compile_expr_with_context(
                stmt, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;
        }

        let current_var = builder.use_var(loop_var);
        let one = builder.ins().iconst(types::I64, 1);
        let incremented = builder.ins().iadd(current_var, one);
        builder.def_var(loop_var, incremented);

        builder.ins().jump(loop_header, &[]);
        builder.seal_block(loop_header);

        builder.switch_to_block(loop_exit);
        builder.seal_block(loop_exit);

        let result = builder.ins().iconst(types::I64, 0);
        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_if_expr(
        condition: &AstNode,
        then_branch: &[AstNode],
        else_branch: &Option<Vec<AstNode>>,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let then_block = builder.create_block();
        let else_block = builder.create_block();
        let merge_block = builder.create_block();

        builder.append_block_param(merge_block, types::I64);

        let cond_value = Self::compile_expr_with_context(
            condition, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let zero = builder.ins().iconst(types::I64, 0);
        let is_true = builder.ins().icmp(IntCC::NotEqual, cond_value, zero);

        builder
            .ins()
            .brif(is_true, then_block, &[], else_block, &[]);

        builder.switch_to_block(then_block);
        builder.seal_block(then_block);

        let then_result = if then_branch.is_empty() {
            builder.ins().iconst(types::I64, 0)
        } else {
            let mut result = builder.ins().iconst(types::I64, 0);
            for stmt in then_branch {
                result = Self::compile_expr_with_context(
                    stmt, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
                )?;
            }
            result
        };

        let then_has_return = !then_branch.is_empty()
            && matches!(then_branch.last().unwrap(), AstNode::Return { .. });
        if !then_has_return {
            builder
                .ins()
                .jump(merge_block, &[BlockArg::Value(then_result)]);
        }

        builder.switch_to_block(else_block);
        builder.seal_block(else_block);

        let else_result = if let Some(else_stmts) = else_branch {
            if else_stmts.is_empty() {
                builder.ins().iconst(types::I64, 0)
            } else {
                let mut result = builder.ins().iconst(types::I64, 0);
                for stmt in else_stmts {
                    result = Self::compile_expr_with_context(
                        stmt, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
                    )?;
                }
                result
            }
        } else {
            builder.ins().iconst(types::I64, 0)
        };

        let else_has_return = if let Some(else_stmts) = else_branch {
            !else_stmts.is_empty()
                && matches!(else_stmts.last().unwrap(), AstNode::Return { .. })
        } else {
            false
        };
        if !else_has_return {
            builder
                .ins()
                .jump(merge_block, &[BlockArg::Value(else_result)]);
        }

        builder.switch_to_block(merge_block);
        builder.seal_block(merge_block);

        let result = builder.block_params(merge_block)[0];

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_function_call(
        name: &str,
        args: &[AstNode],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let func_ptr = compiled_functions.get(name).ok_or_else(|| {
            JitError::UnsupportedNode(format!("Function '{}' not registered", name))
        })?;

        let mut arg_values = Vec::new();
        for arg in args {
            let arg_value = Self::compile_expr_with_context(
                arg, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;
            arg_values.push(arg_value);
        }

        let mut sig = Signature::new(builder.func.signature.call_conv);
        for _ in 0..args.len() {
            sig.params.push(AbiParam::new(types::I64));
        }
        sig.returns.push(AbiParam::new(types::I64));

        let ptr_value = *func_ptr as i64;
        let func_addr = builder.ins().iconst(types::I64, ptr_value);

        let sig_ref = builder.import_signature(sig);
        let call = builder.ins().call_indirect(sig_ref, func_addr, &arg_values);
        let result = builder.inst_results(call)[0];

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_vector_literal(
        elements: &[AstNode],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        if elements.is_empty() {
            return Ok(builder.ins().iconst(types::I64, 0));
        }

        let array_size = elements.len() * 8;
        let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            array_size as u32,
            3,
        ));

        let array_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

        for (i, elem) in elements.iter().enumerate() {
            let elem_value = Self::compile_expr_with_context(
                elem, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;

            let offset = (i * 8) as i32;

            builder
                .ins()
                .store(MemFlags::trusted(), elem_value, array_addr, offset);
        }

        Ok(array_addr)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_tuple_literal(
        elements: &[AstNode],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        if elements.is_empty() {
            return Ok(builder.ins().iconst(types::I64, 0));
        }

        let tuple_size = elements.len() * 8;
        let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            tuple_size as u32,
            3,
        ));

        let tuple_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

        for (i, elem) in elements.iter().enumerate() {
            let elem_value = Self::compile_expr_with_context(
                elem, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;

            let offset = (i * 8) as i32;

            builder
                .ins()
                .store(MemFlags::trusted(), elem_value, tuple_addr, offset);
        }

        Ok(tuple_addr)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_index_access(
        expr: &AstNode,
        index: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let coll_addr = Self::compile_expr_with_context(
            expr, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let key_value = Self::compile_expr_with_context(
            index, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let magic_value = builder
            .ins()
            .load(types::I64, MemFlags::trusted(), coll_addr, 0);

        let magic_constant = builder.ins().iconst(types::I64, -9999);
        let is_hashmap = builder
            .ins()
            .icmp(IntCC::Equal, magic_value, magic_constant);

        let hashmap_block = builder.create_block();
        let array_block = builder.create_block();
        let merge_block = builder.create_block();

        builder.append_block_param(merge_block, types::I64);

        builder
            .ins()
            .brif(is_hashmap, hashmap_block, &[], array_block, &[]);

        builder.switch_to_block(hashmap_block);
        builder.seal_block(hashmap_block);
        {
            let count = builder
                .ins()
                .load(types::I64, MemFlags::trusted(), coll_addr, 8);

            let loop_header = builder.create_block();
            let loop_body = builder.create_block();
            let loop_exit = builder.create_block();

            builder.append_block_param(loop_header, types::I64);

            let i_init = builder.ins().iconst(types::I64, 0);
            builder.ins().jump(loop_header, &[BlockArg::Value(i_init)]);

            builder.switch_to_block(loop_header);
            let i = builder.block_params(loop_header)[0];
            let i_lt_count = builder.ins().icmp(IntCC::SignedLessThan, i, count);
            builder
                .ins()
                .brif(i_lt_count, loop_body, &[], loop_exit, &[]);

            builder.switch_to_block(loop_body);
            builder.seal_block(loop_body);

            let one = builder.ins().iconst(types::I64, 1);
            let two = builder.ins().iconst(types::I64, 2);
            let eight = builder.ins().iconst(types::I64, 8);

            let i_times_2 = builder.ins().imul(i, two);
            let key_index = builder.ins().iadd(two, i_times_2);
            let key_offset = builder.ins().imul(key_index, eight);
            let key_addr = builder.ins().iadd(coll_addr, key_offset);
            let stored_key =
                builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), key_addr, 0);

            let keys_match = builder.ins().icmp(IntCC::Equal, stored_key, key_value);

            let found_block = builder.create_block();
            let continue_block = builder.create_block();
            builder
                .ins()
                .brif(keys_match, found_block, &[], continue_block, &[]);

            builder.switch_to_block(found_block);
            builder.seal_block(found_block);
            let val_index = builder.ins().iadd(key_index, one);
            let val_offset = builder.ins().imul(val_index, eight);
            let val_addr = builder.ins().iadd(coll_addr, val_offset);
            let found_value =
                builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), val_addr, 0);
            builder
                .ins()
                .jump(merge_block, &[BlockArg::Value(found_value)]);

            builder.switch_to_block(continue_block);
            builder.seal_block(continue_block);
            let i_next = builder.ins().iadd(i, one);
            builder.ins().jump(loop_header, &[BlockArg::Value(i_next)]);

            builder.seal_block(loop_header);

            builder.switch_to_block(loop_exit);
            builder.seal_block(loop_exit);
            let not_found = builder.ins().iconst(types::I64, 0);
            builder
                .ins()
                .jump(merge_block, &[BlockArg::Value(not_found)]);
        }

        builder.switch_to_block(array_block);
        builder.seal_block(array_block);
        {
            let eight = builder.ins().iconst(types::I64, 8);
            let byte_offset = builder.ins().imul(key_value, eight);
            let elem_addr = builder.ins().iadd(coll_addr, byte_offset);
            let value = builder
                .ins()
                .load(types::I64, MemFlags::trusted(), elem_addr, 0);
            builder.ins().jump(merge_block, &[BlockArg::Value(value)]);
        }

        builder.switch_to_block(merge_block);
        builder.seal_block(merge_block);
        let result = builder.block_params(merge_block)[0];

        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_compound_assignment(
        lhs: &AstNode,
        op: &BinaryOperator,
        rhs: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        match lhs {
            AstNode::IndexAccess { expr, index } => {
                Self::compile_compound_assign_index(expr, index, op, rhs, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            AstNode::Identifier(name) => {
                Self::compile_compound_assign_var(name, op, rhs, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs)
            }
            _ => Err(JitError::UnsupportedNode(format!(
                "Unsupported LHS in compound assignment: {:?}",
                lhs
            ))),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_compound_assign_index(
        expr: &AstNode,
        index: &AstNode,
        op: &BinaryOperator,
        rhs: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let array_addr = Self::compile_expr_with_context(
            expr, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let index_value = Self::compile_expr_with_context(
            index, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let eight = builder.ins().iconst(types::I64, 8);
        let byte_offset = builder.ins().imul(index_value, eight);
        let elem_addr = builder.ins().iadd(array_addr, byte_offset);

        let current_value =
            builder
                .ins()
                .load(types::I64, MemFlags::trusted(), elem_addr, 0);

        let rhs_value = Self::compile_expr_with_context(
            rhs, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let new_value = Self::apply_compound_op(op, current_value, rhs_value, builder)?;

        builder
            .ins()
            .store(MemFlags::trusted(), new_value, elem_addr, 0);

        Ok(builder.ins().iconst(types::I64, 0))
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_compound_assign_var(
        name: &str,
        op: &BinaryOperator,
        rhs: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let var = *local_vars.get(name).ok_or_else(|| {
            JitError::UnsupportedNode(format!("Undefined variable: {}", name))
        })?;

        let current_value = builder.use_var(var);

        let rhs_value = Self::compile_expr_with_context(
            rhs, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let new_value = Self::apply_compound_op(op, current_value, rhs_value, builder)?;

        builder.def_var(var, new_value);

        Ok(builder.ins().iconst(types::I64, 0))
    }

    fn apply_compound_op(
        op: &BinaryOperator,
        current_value: Value,
        rhs_value: Value,
        builder: &mut FunctionBuilder,
    ) -> Result<Value, JitError> {
        match op {
            BinaryOperator::Add => Ok(builder.ins().iadd(current_value, rhs_value)),
            BinaryOperator::Subtract => Ok(builder.ins().isub(current_value, rhs_value)),
            BinaryOperator::Multiply => Ok(builder.ins().imul(current_value, rhs_value)),
            BinaryOperator::Divide => Ok(builder.ins().sdiv(current_value, rhs_value)),
            BinaryOperator::Modulo => Ok(builder.ins().srem(current_value, rhs_value)),
            _ => {
                Err(JitError::UnsupportedNode(format!(
                    "Unsupported compound assignment operator: {:?}",
                    op
                )))
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_field_access(
        expr: &AstNode,
        field: &str,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let tuple_addr = Self::compile_expr_with_context(
            expr, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        if let Ok(field_index) = field.parse::<usize>() {
            let offset = (field_index * 8) as i32;

            let value =
                builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), tuple_addr, offset);

            Ok(value)
        } else {
            let mut field_offset: Option<i32> = None;

            for (_struct_name, field_names) in struct_defs.iter() {
                if let Some(index) = field_names.iter().position(|f| f == field) {
                    field_offset = Some((index * 8) as i32);
                    break;
                }
            }

            match field_offset {
                Some(offset) => {
                    let value = builder.ins().load(
                        types::I64,
                        MemFlags::trusted(),
                        tuple_addr,
                        offset,
                    );
                    Ok(value)
                }
                None => Err(JitError::UnsupportedNode(format!(
                    "Field '{}' not found in any registered struct",
                    field
                ))),
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_struct_literal(
        name: &str,
        fields: &[(String, AstNode)],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let field_order = struct_defs
            .get(name)
            .ok_or_else(|| {
                JitError::UnsupportedNode(format!("Undefined struct type: {}", name))
            })?
            .clone();

        if fields.is_empty() {
            return Ok(builder.ins().iconst(types::I64, 0));
        }

        let struct_size = field_order.len() * 8;
        let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            struct_size as u32,
            3,
        ));

        let struct_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

        for (field_name, field_value_ast) in fields {
            let field_index = field_order
                .iter()
                .position(|f| f == field_name)
                .ok_or_else(|| {
                    JitError::UnsupportedNode(format!(
                        "Field '{}' not found in struct '{}'",
                        field_name, name
                    ))
                })?;

            let field_value = Self::compile_expr_with_context(
                field_value_ast, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;

            let offset = (field_index * 8) as i32;

            builder
                .ins()
                .store(MemFlags::trusted(), field_value, struct_addr, offset);
        }

        Ok(struct_addr)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_hashmap_literal(
        pairs: &[(AstNode, AstNode)],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        if pairs.is_empty() {
            let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
                StackSlotKind::ExplicitSlot,
                16,
                3,
            ));
            let hashmap_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);
            let magic = builder.ins().iconst(types::I64, -9999);
            let zero = builder.ins().iconst(types::I64, 0);
            builder
                .ins()
                .store(MemFlags::trusted(), magic, hashmap_addr, 0);
            builder
                .ins()
                .store(MemFlags::trusted(), zero, hashmap_addr, 8);
            return Ok(hashmap_addr);
        }

        let hashmap_size = (2 + pairs.len() * 2) * 8;

        let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            hashmap_size as u32,
            3,
        ));

        let hashmap_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

        let magic = builder.ins().iconst(types::I64, -9999);
        builder
            .ins()
            .store(MemFlags::trusted(), magic, hashmap_addr, 0);

        let count = builder.ins().iconst(types::I64, pairs.len() as i64);
        builder
            .ins()
            .store(MemFlags::trusted(), count, hashmap_addr, 8);

        for (i, (key_ast, value_ast)) in pairs.iter().enumerate() {
            let key_value = Self::compile_expr_with_context(
                key_ast, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;

            let val_value = Self::compile_expr_with_context(
                value_ast, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;

            let key_offset = ((2 + i * 2) * 8) as i32;
            let val_offset = ((2 + i * 2 + 1) * 8) as i32;

            builder
                .ins()
                .store(MemFlags::trusted(), key_value, hashmap_addr, key_offset);
            builder
                .ins()
                .store(MemFlags::trusted(), val_value, hashmap_addr, val_offset);
        }

        Ok(hashmap_addr)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_match_expr(
        expr: &AstNode,
        arms: &[crate::interpreter::parser::MatchArm],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let match_value = Self::compile_expr_with_context(
            expr, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let merge_block = builder.create_block();
        builder.append_block_param(merge_block, types::I64);

        let mut current_block = None;
        for (arm_index, arm) in arms.iter().enumerate() {
            let arm_body_block = builder.create_block();
            let next_arm_block = if arm_index < arms.len() - 1 {
                Some(builder.create_block())
            } else {
                None
            };

            if let Some(block) = current_block {
                builder.switch_to_block(block);
            }

            match &arm.pattern {
                Pattern::Literal(pattern_ast) => {
                    let pattern_value = Self::compile_expr_with_context(
                        pattern_ast, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
                    )?;

                    let matches =
                        builder.ins().icmp(IntCC::Equal, match_value, pattern_value);

                    if let Some(next) = next_arm_block {
                        builder.ins().brif(matches, arm_body_block, &[], next, &[]);
                    } else {
                        builder.ins().jump(arm_body_block, &[]);
                    }
                }
                Pattern::Identifier(_name) => {
                    builder.ins().jump(arm_body_block, &[]);
                }
                Pattern::Wildcard => {
                    builder.ins().jump(arm_body_block, &[]);
                }
            }

            if let Some(block) = current_block {
                builder.seal_block(block);
            }

            builder.switch_to_block(arm_body_block);
            builder.seal_block(arm_body_block);

            if let Pattern::Identifier(name) = &arm.pattern {
                let value_type = builder.func.dfg.value_type(match_value);
                let var = builder.declare_var(value_type);
                builder.def_var(var, match_value);
                local_vars.insert(name.clone(), var);
            }

            let arm_result = if arm.body.is_empty() {
                builder.ins().iconst(types::I64, 0)
            } else {
                let mut result = builder.ins().iconst(types::I64, 0);
                for stmt in &arm.body {
                    result = Self::compile_expr_with_context(
                        stmt, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
                    )?;
                }
                result
            };

            builder
                .ins()
                .jump(merge_block, &[BlockArg::Value(arm_result)]);

            current_block = next_arm_block;
        }

        builder.switch_to_block(merge_block);
        builder.seal_block(merge_block);
        let result = builder.block_params(merge_block)[0];
        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_method_call(
        receiver: &AstNode,
        method: &str,
        args: &[AstNode],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let func_ptr = compiled_functions.get(method).ok_or_else(|| {
            JitError::UnsupportedNode(format!("Method '{}' not registered", method))
        })?;

        let receiver_value = Self::compile_expr_with_context(
            receiver, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let mut all_args = vec![receiver_value];
        for arg in args {
            let arg_value = Self::compile_expr_with_context(
                arg, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;
            all_args.push(arg_value);
        }

        let mut sig = Signature::new(builder.func.signature.call_conv);
        for _ in 0..all_args.len() {
            sig.params.push(AbiParam::new(types::I64));
        }
        sig.returns.push(AbiParam::new(types::I64));

        let ptr_value = *func_ptr as i64;
        let func_addr = builder.ins().iconst(types::I64, ptr_value);

        let sig_ref = builder.import_signature(sig);
        let call = builder.ins().call_indirect(sig_ref, func_addr, &all_args);
        let result = builder.inst_results(call)[0];
        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_tuple_destruct(
        names: &[String],
        value: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let tuple_addr = Self::compile_expr_with_context(
            value, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        for (i, name) in names.iter().enumerate() {
            let offset = (i * 8) as i32;

            let field_value =
                builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), tuple_addr, offset);

            let value_type = builder.func.dfg.value_type(field_value);
            let var = builder.declare_var(value_type);

            builder.def_var(var, field_value);

            local_vars.insert(name.clone(), var);
        }

        Ok(builder.ins().iconst(types::I64, 0))
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_type_cast(
        expr: &AstNode,
        target_type: &str,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let value = Self::compile_expr_with_context(
            expr, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let source_type = builder.func.dfg.value_type(value);

        match target_type {
            "f64" => {
                if source_type == types::I64 {
                    let float_value = builder.ins().fcvt_from_sint(types::F64, value);
                    Ok(float_value)
                } else if source_type == types::F64 {
                    Ok(value)
                } else {
                    Err(JitError::UnsupportedNode(format!(
                        "Cannot cast {:?} to f64",
                        source_type
                    )))
                }
            }
            "i64" => {
                if source_type == types::F64 {
                    let int_value = builder.ins().fcvt_to_sint_sat(types::I64, value);
                    Ok(int_value)
                } else if source_type == types::I64 {
                    Ok(value)
                } else {
                    Err(JitError::UnsupportedNode(format!(
                        "Cannot cast {:?} to i64",
                        source_type
                    )))
                }
            }
            _ => Err(JitError::UnsupportedNode(format!(
                "Unknown target type for cast: {}",
                target_type
            ))),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_vec_macro(
        elements: &[AstNode],
        repeat_count: &Option<Box<AstNode>>,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        match repeat_count {
            None => Self::compile_vec_macro_list(elements, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs),
            Some(count_expr) => Self::compile_vec_macro_repeat(elements, count_expr, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_vec_macro_list(
        elements: &[AstNode],
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        if elements.is_empty() {
            let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
                StackSlotKind::ExplicitSlot,
                8,
                3,
            ));
            let array_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);
            return Ok(array_addr);
        }

        let array_size = elements.len() * 8;
        let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            array_size as u32,
            3,
        ));

        let array_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

        for (i, elem) in elements.iter().enumerate() {
            let elem_value = Self::compile_expr_with_context(
                elem, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
            )?;

            let offset = (i * 8) as i32;

            builder.ins().store(
                MemFlags::trusted(),
                elem_value,
                array_addr,
                offset,
            );
        }

        Ok(array_addr)
    }

    #[allow(clippy::too_many_arguments)]
    fn compile_vec_macro_repeat(
        elements: &[AstNode],
        count_expr: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
    ) -> Result<Value, JitError> {
        let value = Self::compile_expr_with_context(
            &elements[0], builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let count = Self::compile_expr_with_context(
            count_expr, builder, parameters, local_vars, var_counter, compiled_functions, string_ctx, struct_defs,
        )?;

        let max_size = 1024 * 8;
        let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            max_size,
            3,
        ));

        let array_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

        let loop_header = builder.create_block();
        let loop_body = builder.create_block();
        let loop_exit = builder.create_block();

        builder.append_block_param(loop_header, types::I64);

        let i_init = builder.ins().iconst(types::I64, 0);
        builder.ins().jump(loop_header, &[BlockArg::Value(i_init)]);

        builder.switch_to_block(loop_header);
        let i = builder.block_params(loop_header)[0];
        let i_lt_count = builder.ins().icmp(IntCC::SignedLessThan, i, count);
        builder
            .ins()
            .brif(i_lt_count, loop_body, &[], loop_exit, &[]);

        builder.switch_to_block(loop_body);
        builder.seal_block(loop_body);

        let eight = builder.ins().iconst(types::I64, 8);
        let byte_offset = builder.ins().imul(i, eight);
        let elem_addr = builder.ins().iadd(array_addr, byte_offset);

        builder
            .ins()
            .store(MemFlags::trusted(), value, elem_addr, 0);

        let one = builder.ins().iconst(types::I64, 1);
        let i_next = builder.ins().iadd(i, one);
        builder.ins().jump(loop_header, &[BlockArg::Value(i_next)]);

        builder.seal_block(loop_header);

        builder.switch_to_block(loop_exit);
        builder.seal_block(loop_exit);

        Ok(array_addr)
    }
}
