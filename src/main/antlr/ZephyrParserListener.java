// Generated from src/main/antlr/ZephyrParser.g4 by ANTLR 4.10.1
import org.antlr.v4.runtime.tree.ParseTreeListener;

/**
 * This interface defines a complete listener for a parse tree produced by
 * {@link ZephyrParser}.
 */
public interface ZephyrParserListener extends ParseTreeListener {
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#singleExpr}.
	 * @param ctx the parse tree
	 */
	void enterSingleExpr(ZephyrParser.SingleExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#singleExpr}.
	 * @param ctx the parse tree
	 */
	void exitSingleExpr(ZephyrParser.SingleExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#module}.
	 * @param ctx the parse tree
	 */
	void enterModule(ZephyrParser.ModuleContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#module}.
	 * @param ctx the parse tree
	 */
	void exitModule(ZephyrParser.ModuleContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#topLevel}.
	 * @param ctx the parse tree
	 */
	void enterTopLevel(ZephyrParser.TopLevelContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#topLevel}.
	 * @param ctx the parse tree
	 */
	void exitTopLevel(ZephyrParser.TopLevelContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignmentExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterAssignmentExpr(ZephyrParser.AssignmentExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignmentExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitAssignmentExpr(ZephyrParser.AssignmentExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code integerExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterIntegerExpr(ZephyrParser.IntegerExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code integerExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitIntegerExpr(ZephyrParser.IntegerExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code objectExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterObjectExpr(ZephyrParser.ObjectExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code objectExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitObjectExpr(ZephyrParser.ObjectExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code castExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterCastExpr(ZephyrParser.CastExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code castExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitCastExpr(ZephyrParser.CastExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code typeCheckExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterTypeCheckExpr(ZephyrParser.TypeCheckExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code typeCheckExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitTypeCheckExpr(ZephyrParser.TypeCheckExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code letExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterLetExpr(ZephyrParser.LetExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code letExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitLetExpr(ZephyrParser.LetExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code stringExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterStringExpr(ZephyrParser.StringExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code stringExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitStringExpr(ZephyrParser.StringExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code identExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterIdentExpr(ZephyrParser.IdentExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code identExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitIdentExpr(ZephyrParser.IdentExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code floatExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterFloatExpr(ZephyrParser.FloatExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code floatExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitFloatExpr(ZephyrParser.FloatExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code forExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterForExpr(ZephyrParser.ForExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code forExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitForExpr(ZephyrParser.ForExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code indexAccessExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterIndexAccessExpr(ZephyrParser.IndexAccessExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code indexAccessExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitIndexAccessExpr(ZephyrParser.IndexAccessExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code codeBlockExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterCodeBlockExpr(ZephyrParser.CodeBlockExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code codeBlockExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitCodeBlockExpr(ZephyrParser.CodeBlockExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code memberAccessExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterMemberAccessExpr(ZephyrParser.MemberAccessExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code memberAccessExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitMemberAccessExpr(ZephyrParser.MemberAccessExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code ifExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterIfExpr(ZephyrParser.IfExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code ifExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitIfExpr(ZephyrParser.IfExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code callExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterCallExpr(ZephyrParser.CallExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code callExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitCallExpr(ZephyrParser.CallExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code listExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterListExpr(ZephyrParser.ListExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code listExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitListExpr(ZephyrParser.ListExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code prefixOp}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterPrefixOp(ZephyrParser.PrefixOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code prefixOp}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitPrefixOp(ZephyrParser.PrefixOpContext ctx);
	/**
	 * Enter a parse tree produced by the {@code charExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterCharExpr(ZephyrParser.CharExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code charExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitCharExpr(ZephyrParser.CharExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code binaryOp}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterBinaryOp(ZephyrParser.BinaryOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code binaryOp}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitBinaryOp(ZephyrParser.BinaryOpContext ctx);
	/**
	 * Enter a parse tree produced by the {@code closureExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterClosureExpr(ZephyrParser.ClosureExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code closureExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitClosureExpr(ZephyrParser.ClosureExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code booleanExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterBooleanExpr(ZephyrParser.BooleanExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code booleanExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitBooleanExpr(ZephyrParser.BooleanExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code mapExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterMapExpr(ZephyrParser.MapExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code mapExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitMapExpr(ZephyrParser.MapExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code parensExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterParensExpr(ZephyrParser.ParensExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code parensExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitParensExpr(ZephyrParser.ParensExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code whileExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterWhileExpr(ZephyrParser.WhileExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code whileExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitWhileExpr(ZephyrParser.WhileExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code postfixOp}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterPostfixOp(ZephyrParser.PostfixOpContext ctx);
	/**
	 * Exit a parse tree produced by the {@code postfixOp}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitPostfixOp(ZephyrParser.PostfixOpContext ctx);
	/**
	 * Enter a parse tree produced by the {@code tupleExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterTupleExpr(ZephyrParser.TupleExprContext ctx);
	/**
	 * Exit a parse tree produced by the {@code tupleExpr}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitTupleExpr(ZephyrParser.TupleExprContext ctx);
	/**
	 * Enter a parse tree produced by the {@code importExpression}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void enterImportExpression(ZephyrParser.ImportExpressionContext ctx);
	/**
	 * Exit a parse tree produced by the {@code importExpression}
	 * labeled alternative in {@link ZephyrParser#expr}.
	 * @param ctx the parse tree
	 */
	void exitImportExpression(ZephyrParser.ImportExpressionContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#term}.
	 * @param ctx the parse tree
	 */
	void enterTerm(ZephyrParser.TermContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#term}.
	 * @param ctx the parse tree
	 */
	void exitTerm(ZephyrParser.TermContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#float}.
	 * @param ctx the parse tree
	 */
	void enterFloat(ZephyrParser.FloatContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#float}.
	 * @param ctx the parse tree
	 */
	void exitFloat(ZephyrParser.FloatContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#integer}.
	 * @param ctx the parse tree
	 */
	void enterInteger(ZephyrParser.IntegerContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#integer}.
	 * @param ctx the parse tree
	 */
	void exitInteger(ZephyrParser.IntegerContext ctx);
	/**
	 * Enter a parse tree produced by the {@code decimal}
	 * labeled alternative in {@link ZephyrParser#intNum}.
	 * @param ctx the parse tree
	 */
	void enterDecimal(ZephyrParser.DecimalContext ctx);
	/**
	 * Exit a parse tree produced by the {@code decimal}
	 * labeled alternative in {@link ZephyrParser#intNum}.
	 * @param ctx the parse tree
	 */
	void exitDecimal(ZephyrParser.DecimalContext ctx);
	/**
	 * Enter a parse tree produced by the {@code binary}
	 * labeled alternative in {@link ZephyrParser#intNum}.
	 * @param ctx the parse tree
	 */
	void enterBinary(ZephyrParser.BinaryContext ctx);
	/**
	 * Exit a parse tree produced by the {@code binary}
	 * labeled alternative in {@link ZephyrParser#intNum}.
	 * @param ctx the parse tree
	 */
	void exitBinary(ZephyrParser.BinaryContext ctx);
	/**
	 * Enter a parse tree produced by the {@code octal}
	 * labeled alternative in {@link ZephyrParser#intNum}.
	 * @param ctx the parse tree
	 */
	void enterOctal(ZephyrParser.OctalContext ctx);
	/**
	 * Exit a parse tree produced by the {@code octal}
	 * labeled alternative in {@link ZephyrParser#intNum}.
	 * @param ctx the parse tree
	 */
	void exitOctal(ZephyrParser.OctalContext ctx);
	/**
	 * Enter a parse tree produced by the {@code hex}
	 * labeled alternative in {@link ZephyrParser#intNum}.
	 * @param ctx the parse tree
	 */
	void enterHex(ZephyrParser.HexContext ctx);
	/**
	 * Exit a parse tree produced by the {@code hex}
	 * labeled alternative in {@link ZephyrParser#intNum}.
	 * @param ctx the parse tree
	 */
	void exitHex(ZephyrParser.HexContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#boolean}.
	 * @param ctx the parse tree
	 */
	void enterBoolean(ZephyrParser.BooleanContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#boolean}.
	 * @param ctx the parse tree
	 */
	void exitBoolean(ZephyrParser.BooleanContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#list}.
	 * @param ctx the parse tree
	 */
	void enterList(ZephyrParser.ListContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#list}.
	 * @param ctx the parse tree
	 */
	void exitList(ZephyrParser.ListContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#map}.
	 * @param ctx the parse tree
	 */
	void enterMap(ZephyrParser.MapContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#map}.
	 * @param ctx the parse tree
	 */
	void exitMap(ZephyrParser.MapContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#tuple}.
	 * @param ctx the parse tree
	 */
	void enterTuple(ZephyrParser.TupleContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#tuple}.
	 * @param ctx the parse tree
	 */
	void exitTuple(ZephyrParser.TupleContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#object}.
	 * @param ctx the parse tree
	 */
	void enterObject(ZephyrParser.ObjectContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#object}.
	 * @param ctx the parse tree
	 */
	void exitObject(ZephyrParser.ObjectContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#mapEntry}.
	 * @param ctx the parse tree
	 */
	void enterMapEntry(ZephyrParser.MapEntryContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#mapEntry}.
	 * @param ctx the parse tree
	 */
	void exitMapEntry(ZephyrParser.MapEntryContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#objectField}.
	 * @param ctx the parse tree
	 */
	void enterObjectField(ZephyrParser.ObjectFieldContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#objectField}.
	 * @param ctx the parse tree
	 */
	void exitObjectField(ZephyrParser.ObjectFieldContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#closure}.
	 * @param ctx the parse tree
	 */
	void enterClosure(ZephyrParser.ClosureContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#closure}.
	 * @param ctx the parse tree
	 */
	void exitClosure(ZephyrParser.ClosureContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#closureParam}.
	 * @param ctx the parse tree
	 */
	void enterClosureParam(ZephyrParser.ClosureParamContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#closureParam}.
	 * @param ctx the parse tree
	 */
	void exitClosureParam(ZephyrParser.ClosureParamContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#ident}.
	 * @param ctx the parse tree
	 */
	void enterIdent(ZephyrParser.IdentContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#ident}.
	 * @param ctx the parse tree
	 */
	void exitIdent(ZephyrParser.IdentContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#call}.
	 * @param ctx the parse tree
	 */
	void enterCall(ZephyrParser.CallContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#call}.
	 * @param ctx the parse tree
	 */
	void exitCall(ZephyrParser.CallContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#let}.
	 * @param ctx the parse tree
	 */
	void enterLet(ZephyrParser.LetContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#let}.
	 * @param ctx the parse tree
	 */
	void exitLet(ZephyrParser.LetContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#external}.
	 * @param ctx the parse tree
	 */
	void enterExternal(ZephyrParser.ExternalContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#external}.
	 * @param ctx the parse tree
	 */
	void exitExternal(ZephyrParser.ExternalContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignIdent}
	 * labeled alternative in {@link ZephyrParser#assignment}.
	 * @param ctx the parse tree
	 */
	void enterAssignIdent(ZephyrParser.AssignIdentContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignIdent}
	 * labeled alternative in {@link ZephyrParser#assignment}.
	 * @param ctx the parse tree
	 */
	void exitAssignIdent(ZephyrParser.AssignIdentContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignMember}
	 * labeled alternative in {@link ZephyrParser#assignment}.
	 * @param ctx the parse tree
	 */
	void enterAssignMember(ZephyrParser.AssignMemberContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignMember}
	 * labeled alternative in {@link ZephyrParser#assignment}.
	 * @param ctx the parse tree
	 */
	void exitAssignMember(ZephyrParser.AssignMemberContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignIndex}
	 * labeled alternative in {@link ZephyrParser#assignment}.
	 * @param ctx the parse tree
	 */
	void enterAssignIndex(ZephyrParser.AssignIndexContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignIndex}
	 * labeled alternative in {@link ZephyrParser#assignment}.
	 * @param ctx the parse tree
	 */
	void exitAssignIndex(ZephyrParser.AssignIndexContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignNone}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void enterAssignNone(ZephyrParser.AssignNoneContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignNone}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void exitAssignNone(ZephyrParser.AssignNoneContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignPlus}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void enterAssignPlus(ZephyrParser.AssignPlusContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignPlus}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void exitAssignPlus(ZephyrParser.AssignPlusContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignMinus}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void enterAssignMinus(ZephyrParser.AssignMinusContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignMinus}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void exitAssignMinus(ZephyrParser.AssignMinusContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignTimes}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void enterAssignTimes(ZephyrParser.AssignTimesContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignTimes}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void exitAssignTimes(ZephyrParser.AssignTimesContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignDivide}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void enterAssignDivide(ZephyrParser.AssignDivideContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignDivide}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void exitAssignDivide(ZephyrParser.AssignDivideContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignModulus}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void enterAssignModulus(ZephyrParser.AssignModulusContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignModulus}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void exitAssignModulus(ZephyrParser.AssignModulusContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignPower}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void enterAssignPower(ZephyrParser.AssignPowerContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignPower}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void exitAssignPower(ZephyrParser.AssignPowerContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignAnd}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void enterAssignAnd(ZephyrParser.AssignAndContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignAnd}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void exitAssignAnd(ZephyrParser.AssignAndContext ctx);
	/**
	 * Enter a parse tree produced by the {@code assignOr}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void enterAssignOr(ZephyrParser.AssignOrContext ctx);
	/**
	 * Exit a parse tree produced by the {@code assignOr}
	 * labeled alternative in {@link ZephyrParser#assignmentOp}.
	 * @param ctx the parse tree
	 */
	void exitAssignOr(ZephyrParser.AssignOrContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#memberAccess}.
	 * @param ctx the parse tree
	 */
	void enterMemberAccess(ZephyrParser.MemberAccessContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#memberAccess}.
	 * @param ctx the parse tree
	 */
	void exitMemberAccess(ZephyrParser.MemberAccessContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#indexAccess}.
	 * @param ctx the parse tree
	 */
	void enterIndexAccess(ZephyrParser.IndexAccessContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#indexAccess}.
	 * @param ctx the parse tree
	 */
	void exitIndexAccess(ZephyrParser.IndexAccessContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#if}.
	 * @param ctx the parse tree
	 */
	void enterIf(ZephyrParser.IfContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#if}.
	 * @param ctx the parse tree
	 */
	void exitIf(ZephyrParser.IfContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#while}.
	 * @param ctx the parse tree
	 */
	void enterWhile(ZephyrParser.WhileContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#while}.
	 * @param ctx the parse tree
	 */
	void exitWhile(ZephyrParser.WhileContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#for}.
	 * @param ctx the parse tree
	 */
	void enterFor(ZephyrParser.ForContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#for}.
	 * @param ctx the parse tree
	 */
	void exitFor(ZephyrParser.ForContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#importExpr}.
	 * @param ctx the parse tree
	 */
	void enterImportExpr(ZephyrParser.ImportExprContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#importExpr}.
	 * @param ctx the parse tree
	 */
	void exitImportExpr(ZephyrParser.ImportExprContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#importIdent}.
	 * @param ctx the parse tree
	 */
	void enterImportIdent(ZephyrParser.ImportIdentContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#importIdent}.
	 * @param ctx the parse tree
	 */
	void exitImportIdent(ZephyrParser.ImportIdentContext ctx);
	/**
	 * Enter a parse tree produced by the {@code importPathStd}
	 * labeled alternative in {@link ZephyrParser#importSpecifier}.
	 * @param ctx the parse tree
	 */
	void enterImportPathStd(ZephyrParser.ImportPathStdContext ctx);
	/**
	 * Exit a parse tree produced by the {@code importPathStd}
	 * labeled alternative in {@link ZephyrParser#importSpecifier}.
	 * @param ctx the parse tree
	 */
	void exitImportPathStd(ZephyrParser.ImportPathStdContext ctx);
	/**
	 * Enter a parse tree produced by the {@code importPathLib}
	 * labeled alternative in {@link ZephyrParser#importSpecifier}.
	 * @param ctx the parse tree
	 */
	void enterImportPathLib(ZephyrParser.ImportPathLibContext ctx);
	/**
	 * Exit a parse tree produced by the {@code importPathLib}
	 * labeled alternative in {@link ZephyrParser#importSpecifier}.
	 * @param ctx the parse tree
	 */
	void exitImportPathLib(ZephyrParser.ImportPathLibContext ctx);
	/**
	 * Enter a parse tree produced by the {@code importPathFile}
	 * labeled alternative in {@link ZephyrParser#importSpecifier}.
	 * @param ctx the parse tree
	 */
	void enterImportPathFile(ZephyrParser.ImportPathFileContext ctx);
	/**
	 * Exit a parse tree produced by the {@code importPathFile}
	 * labeled alternative in {@link ZephyrParser#importSpecifier}.
	 * @param ctx the parse tree
	 */
	void exitImportPathFile(ZephyrParser.ImportPathFileContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#importFilePart}.
	 * @param ctx the parse tree
	 */
	void enterImportFilePart(ZephyrParser.ImportFilePartContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#importFilePart}.
	 * @param ctx the parse tree
	 */
	void exitImportFilePart(ZephyrParser.ImportFilePartContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#importFileRegular}.
	 * @param ctx the parse tree
	 */
	void enterImportFileRegular(ZephyrParser.ImportFileRegularContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#importFileRegular}.
	 * @param ctx the parse tree
	 */
	void exitImportFileRegular(ZephyrParser.ImportFileRegularContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#cast}.
	 * @param ctx the parse tree
	 */
	void enterCast(ZephyrParser.CastContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#cast}.
	 * @param ctx the parse tree
	 */
	void exitCast(ZephyrParser.CastContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#typeCheck}.
	 * @param ctx the parse tree
	 */
	void enterTypeCheck(ZephyrParser.TypeCheckContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#typeCheck}.
	 * @param ctx the parse tree
	 */
	void exitTypeCheck(ZephyrParser.TypeCheckContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#codeBlock}.
	 * @param ctx the parse tree
	 */
	void enterCodeBlock(ZephyrParser.CodeBlockContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#codeBlock}.
	 * @param ctx the parse tree
	 */
	void exitCodeBlock(ZephyrParser.CodeBlockContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#parens}.
	 * @param ctx the parse tree
	 */
	void enterParens(ZephyrParser.ParensContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#parens}.
	 * @param ctx the parse tree
	 */
	void exitParens(ZephyrParser.ParensContext ctx);
	/**
	 * Enter a parse tree produced by {@link ZephyrParser#type}.
	 * @param ctx the parse tree
	 */
	void enterType(ZephyrParser.TypeContext ctx);
	/**
	 * Exit a parse tree produced by {@link ZephyrParser#type}.
	 * @param ctx the parse tree
	 */
	void exitType(ZephyrParser.TypeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code intType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void enterIntType(ZephyrParser.IntTypeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code intType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void exitIntType(ZephyrParser.IntTypeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code floatType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void enterFloatType(ZephyrParser.FloatTypeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code floatType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void exitFloatType(ZephyrParser.FloatTypeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code charType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void enterCharType(ZephyrParser.CharTypeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code charType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void exitCharType(ZephyrParser.CharTypeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code stringType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void enterStringType(ZephyrParser.StringTypeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code stringType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void exitStringType(ZephyrParser.StringTypeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code booleanType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void enterBooleanType(ZephyrParser.BooleanTypeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code booleanType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void exitBooleanType(ZephyrParser.BooleanTypeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code voidType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void enterVoidType(ZephyrParser.VoidTypeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code voidType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void exitVoidType(ZephyrParser.VoidTypeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code listType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void enterListType(ZephyrParser.ListTypeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code listType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void exitListType(ZephyrParser.ListTypeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code mapType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void enterMapType(ZephyrParser.MapTypeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code mapType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void exitMapType(ZephyrParser.MapTypeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code tupleType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void enterTupleType(ZephyrParser.TupleTypeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code tupleType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void exitTupleType(ZephyrParser.TupleTypeContext ctx);
	/**
	 * Enter a parse tree produced by the {@code objectType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void enterObjectType(ZephyrParser.ObjectTypeContext ctx);
	/**
	 * Exit a parse tree produced by the {@code objectType}
	 * labeled alternative in {@link ZephyrParser#typeLiteral}.
	 * @param ctx the parse tree
	 */
	void exitObjectType(ZephyrParser.ObjectTypeContext ctx);
}