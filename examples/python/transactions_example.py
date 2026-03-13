from mmex_lib import (
    MmexEngine,
    Transaction,
    TransactionId,
    TransactionCode,
    TransactionStatus,
    AccountId,
    PayeeId,
    CategoryId,
    Money,
    MmexDate,
    MmexError,
)
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
import os
import shutil
from datetime import date


def transactions_demo():
    console = Console()
    base_db = "cp_of_personal_finance.mmb"
    db_path = "example_transactions.mmb"

    if os.path.exists(db_path):
        os.remove(db_path)

    if not os.path.exists(base_db):
        console.print(f"[bold red]Error:[/bold red] Base database {base_db} not found.")
        return

    shutil.copy(base_db, db_path)

    console.print(Panel("[bold yellow]Transactions & Tag Linking Demo[/bold yellow]"))

    try:
        engine = MmexEngine(db_path, None)
        tx_mgr = engine.transactions()
        tag_mgr = engine.tags()

        # 1. Setup prerequisites
        console.print("\n[bold]1. Creating prerequisite tag...[/bold]")
        my_tag = tag_mgr.create("Python Demo Tag")

        # 2. Create a Transaction
        console.print("\n[bold]2. Creating a Transaction...[/bold]")
        tx_data = Transaction(
            id=TransactionId(v1=0),
            account_id=AccountId(v1=1),
            to_account_id=None,
            payee_id=PayeeId(v1=1),
            trans_code=TransactionCode.WITHDRAWAL(),
            amount=Money(v1="123.45"),
            status=TransactionStatus.NONE(),
            transaction_number="TX-999",
            notes="Demo transaction from Python",
            category_id=CategoryId(v1=1),
            date=MmexDate(v1=date.today().isoformat()),
            to_amount=Money(v1="123.45"),
        )

        new_tx = tx_mgr.create(tx_data)
        console.print(
            f"Created Transaction ID: [cyan]{new_tx.id.v1}[/cyan], Amount: [green]{new_tx.amount.v1}[/green]"
        )

        # 3. Link Tag
        console.print(
            f"\n[bold]3. Linking tag '{my_tag.name}' to transaction...[/bold]"
        )
        # Fixed: removed "CheckingAccount" as tx_mgr already knows the type for transactions
        tx_mgr.link_tag(new_tx.id.v1, my_tag.id.v1)
        console.print("Tag linked successfully!")

        # 4. List Transactions
        console.print("\n[bold]4. Latest Transactions:[/bold]")
        all_txs = tx_mgr.get_all()
        table = Table(show_header=True, header_style="bold cyan")
        table.add_column("ID", style="dim")
        table.add_column("Date")
        table.add_column("Notes")
        table.add_column("Amount", justify="right")

        # Show last 5 transactions
        for tx in all_txs[-5:]:
            table.add_row(
                str(tx.id.v1),
                tx.date.v1 if tx.date else "N/A",
                tx.notes or "",
                tx.amount.v1,
            )
        console.print(table)

        # 5. Error Handling
        console.print("\n[bold red]5. Error Handling: Invalid ID[/bold red]")
        try:
            res = tx_mgr.get_by_id(-99)
            if res is None:
                console.print(
                    "[yellow]Correctly handled: Transaction -99 not found[/yellow]"
                )
        except Exception as e:
            console.print(f"[red]Caught exception: {e}[/red]")

    except MmexError as e:
        console.print(f"[bold red]Mmex Error:[/bold red] {e}")
    except Exception as e:
        console.print(f"[bold red]Unexpected Error:[/bold red] {type(e).__name__}: {e}")
    finally:
        if os.path.exists(db_path):
            os.remove(db_path)


if __name__ == "__main__":
    transactions_demo()
