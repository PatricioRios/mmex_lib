import uniffi.mmex_lib.*

fun main() {

    var engine : MmexEngine = MmexEngine(
        path = "personal_finance.mmb",
        null
    );

    engine.getTags().forEachIndexed { index, tag ->
        println(tag)
    }

}
