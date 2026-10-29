// Generated from PS.g4 by ANTLR 4.13.2

use super::psparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by PSParser.

pub trait PSBaseListener<'input>:
    ParseTreeListener<'input, PSParserContextType> {

    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_math(&mut self, _ctx: &MathContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_math(&mut self, _ctx: &MathContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_transpose(&mut self, _ctx: &TransposeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_transpose(&mut self, _ctx: &TransposeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_degree(&mut self, _ctx: &DegreeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_degree(&mut self, _ctx: &DegreeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_transform_atom(&mut self, _ctx: &Transform_atomContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_transform_atom(&mut self, _ctx: &Transform_atomContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_transform_scale(&mut self, _ctx: &Transform_scaleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_transform_scale(&mut self, _ctx: &Transform_scaleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_transform_swap(&mut self, _ctx: &Transform_swapContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_transform_swap(&mut self, _ctx: &Transform_swapContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_transform_assignment(&mut self, _ctx: &Transform_assignmentContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_transform_assignment(&mut self, _ctx: &Transform_assignmentContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementary_transform(&mut self, _ctx: &Elementary_transformContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementary_transform(&mut self, _ctx: &Elementary_transformContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_elementary_transforms(&mut self, _ctx: &Elementary_transformsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_elementary_transforms(&mut self, _ctx: &Elementary_transformsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_matrix(&mut self, _ctx: &MatrixContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_matrix(&mut self, _ctx: &MatrixContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_det(&mut self, _ctx: &DetContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_det(&mut self, _ctx: &DetContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_matrix_row(&mut self, _ctx: &Matrix_rowContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_matrix_row(&mut self, _ctx: &Matrix_rowContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_relation(&mut self, _ctx: &RelationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_relation(&mut self, _ctx: &RelationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_equality(&mut self, _ctx: &EqualityContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_equality(&mut self, _ctx: &EqualityContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expr(&mut self, _ctx: &ExprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expr(&mut self, _ctx: &ExprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_additive(&mut self, _ctx: &AdditiveContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_additive(&mut self, _ctx: &AdditiveContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_mp(&mut self, _ctx: &MpContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mp(&mut self, _ctx: &MpContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_mp_nofunc(&mut self, _ctx: &Mp_nofuncContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_mp_nofunc(&mut self, _ctx: &Mp_nofuncContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unary(&mut self, _ctx: &UnaryContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unary(&mut self, _ctx: &UnaryContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_unary_nofunc(&mut self, _ctx: &Unary_nofuncContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_unary_nofunc(&mut self, _ctx: &Unary_nofuncContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_postfix(&mut self, _ctx: &PostfixContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_postfix(&mut self, _ctx: &PostfixContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_postfix_nofunc(&mut self, _ctx: &Postfix_nofuncContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_postfix_nofunc(&mut self, _ctx: &Postfix_nofuncContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_postfix_op(&mut self, _ctx: &Postfix_opContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_postfix_op(&mut self, _ctx: &Postfix_opContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_eval_at(&mut self, _ctx: &Eval_atContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_eval_at(&mut self, _ctx: &Eval_atContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_eval_at_sub(&mut self, _ctx: &Eval_at_subContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_eval_at_sub(&mut self, _ctx: &Eval_at_subContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_eval_at_sup(&mut self, _ctx: &Eval_at_supContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_eval_at_sup(&mut self, _ctx: &Eval_at_supContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_exp(&mut self, _ctx: &ExpContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_exp(&mut self, _ctx: &ExpContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_exp_nofunc(&mut self, _ctx: &Exp_nofuncContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_exp_nofunc(&mut self, _ctx: &Exp_nofuncContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_comp(&mut self, _ctx: &CompContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_comp(&mut self, _ctx: &CompContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_comp_nofunc(&mut self, _ctx: &Comp_nofuncContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_comp_nofunc(&mut self, _ctx: &Comp_nofuncContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_group(&mut self, _ctx: &GroupContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_group(&mut self, _ctx: &GroupContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_formatting_group(&mut self, _ctx: &Formatting_groupContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_formatting_group(&mut self, _ctx: &Formatting_groupContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_norm_group(&mut self, _ctx: &Norm_groupContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_norm_group(&mut self, _ctx: &Norm_groupContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_abs_group(&mut self, _ctx: &Abs_groupContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_abs_group(&mut self, _ctx: &Abs_groupContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_dot_product(&mut self, _ctx: &Dot_productContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dot_product(&mut self, _ctx: &Dot_productContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_floor_group(&mut self, _ctx: &Floor_groupContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_floor_group(&mut self, _ctx: &Floor_groupContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ceil_group(&mut self, _ctx: &Ceil_groupContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ceil_group(&mut self, _ctx: &Ceil_groupContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atom_expr_no_supexpr(&mut self, _ctx: &Atom_expr_no_supexprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom_expr_no_supexpr(&mut self, _ctx: &Atom_expr_no_supexprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atom_expr(&mut self, _ctx: &Atom_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom_expr(&mut self, _ctx: &Atom_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atom_expr_list(&mut self, _ctx: &Atom_expr_listContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom_expr_list(&mut self, _ctx: &Atom_expr_listContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_number_subexpr(&mut self, _ctx: &Number_subexprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_number_subexpr(&mut self, _ctx: &Number_subexprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_atom(&mut self, _ctx: &AtomContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_atom(&mut self, _ctx: &AtomContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_frac(&mut self, _ctx: &FracContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_frac(&mut self, _ctx: &FracContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_binom(&mut self, _ctx: &BinomContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_binom(&mut self, _ctx: &BinomContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_normal_functions_single_arg(&mut self, _ctx: &Func_normal_functions_single_argContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_normal_functions_single_arg(&mut self, _ctx: &Func_normal_functions_single_argContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_normal_functions_multi_arg(&mut self, _ctx: &Func_normal_functions_multi_argContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_normal_functions_multi_arg(&mut self, _ctx: &Func_normal_functions_multi_argContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_operator_names_single_arg(&mut self, _ctx: &Func_operator_names_single_argContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_operator_names_single_arg(&mut self, _ctx: &Func_operator_names_single_argContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_operator_names_multi_arg(&mut self, _ctx: &Func_operator_names_multi_argContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_operator_names_multi_arg(&mut self, _ctx: &Func_operator_names_multi_argContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_normal_single_arg(&mut self, _ctx: &Func_normal_single_argContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_normal_single_arg(&mut self, _ctx: &Func_normal_single_argContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_normal_multi_arg(&mut self, _ctx: &Func_normal_multi_argContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_normal_multi_arg(&mut self, _ctx: &Func_normal_multi_argContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func(&mut self, _ctx: &FuncContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func(&mut self, _ctx: &FuncContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_args(&mut self, _ctx: &ArgsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_args(&mut self, _ctx: &ArgsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_common_args(&mut self, _ctx: &Func_common_argsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_common_args(&mut self, _ctx: &Func_common_argsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_limit_sub(&mut self, _ctx: &Limit_subContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_limit_sub(&mut self, _ctx: &Limit_subContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_single_arg(&mut self, _ctx: &Func_single_argContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_single_arg(&mut self, _ctx: &Func_single_argContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_single_arg_noparens(&mut self, _ctx: &Func_single_arg_noparensContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_single_arg_noparens(&mut self, _ctx: &Func_single_arg_noparensContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_multi_arg(&mut self, _ctx: &Func_multi_argContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_multi_arg(&mut self, _ctx: &Func_multi_argContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_func_multi_arg_noparens(&mut self, _ctx: &Func_multi_arg_noparensContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_func_multi_arg_noparens(&mut self, _ctx: &Func_multi_arg_noparensContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_subexpr(&mut self, _ctx: &SubexprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_subexpr(&mut self, _ctx: &SubexprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_supexpr(&mut self, _ctx: &SupexprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_supexpr(&mut self, _ctx: &SupexprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_subeq(&mut self, _ctx: &SubeqContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_subeq(&mut self, _ctx: &SubeqContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_supeq(&mut self, _ctx: &SupeqContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_supeq(&mut self, _ctx: &SupeqContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_set_relation(&mut self, _ctx: &Set_relationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_set_relation(&mut self, _ctx: &Set_relationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_minus_expr(&mut self, _ctx: &Minus_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_minus_expr(&mut self, _ctx: &Minus_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_union_expr(&mut self, _ctx: &Union_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_union_expr(&mut self, _ctx: &Union_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_intersection_expr(&mut self, _ctx: &Intersection_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_intersection_expr(&mut self, _ctx: &Intersection_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_set_group(&mut self, _ctx: &Set_groupContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_set_group(&mut self, _ctx: &Set_groupContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_set_atom(&mut self, _ctx: &Set_atomContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_set_atom(&mut self, _ctx: &Set_atomContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_interval(&mut self, _ctx: &IntervalContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_interval(&mut self, _ctx: &IntervalContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_ordered_tuple(&mut self, _ctx: &Ordered_tupleContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_ordered_tuple(&mut self, _ctx: &Ordered_tupleContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_finite_set(&mut self, _ctx: &Finite_setContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_finite_set(&mut self, _ctx: &Finite_setContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_set_elements_relation(&mut self, _ctx: &Set_elements_relationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_set_elements_relation(&mut self, _ctx: &Set_elements_relationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_set_elements(&mut self, _ctx: &Set_elementsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_set_elements(&mut self, _ctx: &Set_elementsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_semicolon_elements(&mut self, _ctx: &Semicolon_elementsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_semicolon_elements(&mut self, _ctx: &Semicolon_elementsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_semicolon_elements_no_relation(&mut self, _ctx: &Semicolon_elements_no_relationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_semicolon_elements_no_relation(&mut self, _ctx: &Semicolon_elements_no_relationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_comma_elements(&mut self, _ctx: &Comma_elementsContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_comma_elements(&mut self, _ctx: &Comma_elementsContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_comma_elements_no_relation(&mut self, _ctx: &Comma_elements_no_relationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_comma_elements_no_relation(&mut self, _ctx: &Comma_elements_no_relationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_element_no_relation(&mut self, _ctx: &Element_no_relationContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_element_no_relation(&mut self, _ctx: &Element_no_relationContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_element(&mut self, _ctx: &ElementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_element(&mut self, _ctx: &ElementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_plus_minus_expr(&mut self, _ctx: &Plus_minus_exprContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_plus_minus_expr(&mut self, _ctx: &Plus_minus_exprContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_literal_set(&mut self, _ctx: &Literal_setContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  PSBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_literal_set(&mut self, _ctx: &Literal_setContext<'input>) {}


}