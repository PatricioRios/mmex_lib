from mmex_lib import (
    MmexEngine,
    MmexError,
    Account,
    AccountId,
    Money,
    CurrencyId,
    AccountType,
    AccountStatus,
)
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
import os
import shutil


def accounts_demo():
    console = Console()
    base_db = "cp_of_personal_finance.mmb"
    db_path = "example_accounts.mmb"

    if os.path.exists(db_path):
        os.remove(db_path)

    if not os.path.exists(base_db):
        console.print(f"[bold red]Error:[/bold red] Base database {base_db} not found.")
        return

    shutil.copy(base_db, db_path)

    console.print(Panel("[bold magenta]Accounts Module Demo[/bold magenta]"))

    try:
        engine = MmexEngine(db_path, None)
        acc_mgr = engine.accounts()

        # 1. Create Account
        console.print("\n[bold]1. Creating Account...[/bold]")
        acc1_data = Account(
            id=AccountId(v1=0),
            name="Python Demo Account",
            account_type=AccountType.CHECKING(),
            account_num="999-888",
            status=AccountStatus.OPEN(),  # Fixed: used OPEN instead of ACTIVE
            notes="Demo account created via Python",
            initial_balance=Money(v1="5000.00"),
            currency_id=CurrencyId(v1=1),
            favorite=True,
        )

        acc1 = acc_mgr.create(acc1_data)
        console.print(f"Created account: [green]{acc1.name}[/green] (ID: {acc1.id.v1})")

        # 2. List all accounts
        console.print("\n[bold]2. Listing all accounts:[/bold]")
        all_accounts = acc_mgr.get_all()
        table = Table(show_header=True, header_style="bold yellow")
        table.add_column("ID", style="dim")
        table.add_column("Name", style="bold")
        table.add_column("Type")
        table.add_column("Balance", justify="right")

        for acc in all_accounts:
            try:
                balance = acc_mgr.get_balance(acc.id.v1)
                table.add_row(
                    str(acc.id.v1),
                    acc.name,
                    str(acc.account_type),
                    f"{balance.current_balance.v1}",
                )
            except Exception as e:
                table.add_row(
                    str(acc.id.v1),
                    acc.name,
                    str(acc.account_type),
                    f"Error: {type(e).__name__}",
                )

        console.print(table)

        # 3. Error Handling: Accessing non-existent account
        console.print(
            "\n[bold red]3. Error Handling: Accessing non-existent account[/bold red]"
        )
        res = acc_mgr.get_by_id(99999)
        if res is None:
            console.print(
                "[yellow]Correctly handled: Account 99999 not found (returned None)[/yellow]"
            )

    except MmexError as e:
        console.print(f"[bold red]Mmex Error:[/bold red] {e}")
    except Exception as e:
        console.print(f"[bold red]Unexpected Error:[/bold red] {type(e).__name__}: {e}")
    finally:
        if os.path.exists(db_path):
            os.remove(db_path)


if __name__ == "__main__":
    accounts_demo()
