import com.theonlytails.zephyr_lang.parser.*
import com.theonlytails.zephyr_lang.parser.ZephyrType.*
import kotlin.test.Test
import kotlin.test.assertEquals

class GrammarTest {
    @Test
    fun `decimal integer`() {
        assertEquals(
            Parser.parse("1", target = ParseTarget.Expr), Result.success(
                ZephyrModule(mapOf("_value" to Declaration(IntType, ZephyrExpression.IntExpr(1))))
            )
        )
    }
}
