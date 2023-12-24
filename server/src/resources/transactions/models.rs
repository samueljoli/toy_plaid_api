use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Transactions {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct ApiResponse {
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
    item: Item,
    total_transactions: u32,
    request_id: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
enum AccountType {
    Investment,
    Credit,
    Depository,
    Loan,
    Other,
}
// TODO: Handle AccountType and SubType validation combo (Runtime)

#[derive(Serialize, Deserialize, Debug, PartialEq, ToSchema)]
enum DepositorySubtype {
    Savings,
    Hsa,
    Cd,
    MoneyMarket,
    Paypal,
    Prepaid,
    CashManagement,
    Ebt,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct Account {
    account_id: String,
    mask: String,
    name: String,
    official_name: String,
    r#type: AccountType,
    subtype: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct Transaction {
    account_id: String, // Foreign key Account
    amount: f64,
    iso_currency_code: String,
    unofficial_currency_code: Option<String>,
    check_number: Option<String>,
    date: String,
    datetime: String,
    authorized_date: String,
    authorized_datetime: String,
    location: Location, // Foreign key to Location
    name: String,
    merchant_name: String,
    merchant_entity_id: String,
    logo_url: String,
    website: String,
    payment_channel: String,
    pending: bool,
    pending_transaction_id: Option<String>,
    personal_finance_category: PersonalFinanceCategory, // Foreign key PersonalFinanceCategory
    personal_finance_category_icon_url: String,
    transaction_id: String,
    transaction_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct Location {
    address: Option<String>,
    city: Option<String>,
    region: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
    store_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
enum Primary {
    Income,
    TransferIn,
    TransferOut,
    LoanPayments,
    BankFees,
    Entertainment,
    FoodAndDrink,
    GeneralMerchandise,
    HomeImprovement,
    Medical,
    PersonalCare,
    GeneralServices,
    GovernmentAndNonProfit,
    Transportation,
    Travel,
    RentAndUtilities,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
enum Detailed {
    IncomeDividends,
    IncomeInterestEarned,
    IncomeRetirementPension,
    IncomeTaxRefund,
    IncomeUnemployment,
    IncomeWages,
    IncomeOtherIncome,
    TransferInCashAdvancesAndLoans,
    TransferInDeposit,
    TransferInInvestmentAndRetirementFunds,
    TransferInSavings,
    TransferInAccountTransfer,
    TransferInOtherTransferIn,
    TransferOutInvestmentAndRetirementFunds,
    TransferOutSavings,
    TransferOutWithdrawal,
    TransferOutAccountTransfer,
    TransferOutOtherTransferOut,
    LoanPaymentsCarPayment,
    LoanPaymentsCreditCardPayment,
    LoanPaymentsPersonalLoanPayment,
    LoanPaymentsMortgagePayment,
    LoanPaymentsStudentLoanPayment,
    LoanPaymentsOtherPayment,
    BankFeesAtmFees,
    BankFeesForeignTransactionFees,
    BankFeesInsufficientFunds,
    BankFeesInterestCharge,
    BankFeesOverdraftFees,
    BankFeesOtherBankFees,
    EntertainmentCasinosAndGambling,
    EntertainmentMusicAndAudio,
    EntertainmentSportingEventsAmusementParksAndMuseums,
    EntertainmentTvAndMovies,
    EntertainmentVideoGames,
    EntertainmentOtherEntertainment,
    FoodAndDrinkBeerWineAndLiquor,
    FoodAndDrinkCoffee,
    FoodAndDrinkFastFood,
    FoodAndDrinkGroceries,
    FoodAndDrinkRestaurant,
    FoodAndDrinkVendingMachines,
    FoodAndDrinkOtherFoodAndDrink,
    GeneralMerchandiseBookstoresAndNewsstands,
    GeneralMerchandiseClothingAndAccessories,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
enum ConfidenceLevel {
    VeryHigh,
    High,
    Medium,
    Low,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct PersonalFinanceCategory {
    primary: Primary,
    detailed: Detailed,
    confidence_level: ConfidenceLevel,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct Item {
    consent_expiration_time: Option<String>,
    error: Option<String>,
    institution_id: String,
    item_id: String,
    update_type: String,
    webhook: String,
}

// JOIN tables
#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct TransactionPersonalFinanceCategory {
    transaction_id: String,               // Foreign key Transaction
    personal_finance_category_id: String, // Foreign key PersonalFinanceCategory
}
