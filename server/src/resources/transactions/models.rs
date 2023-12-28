use sea_query::enum_def;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::resources::accounts::models::Account;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
struct ApiResponse {
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
    total_transactions: u32,
    request_id: String,
}

#[enum_def] // => Generates TransactionIden
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Transaction {
    pub id: i32,
    pub account_id: i32, // Foreign key Account
    pub amount: f64,
    pub iso_currency_code: String,
    pub date: String,
    pub name: String,
    pub merchant_name: String,
    pub payment_channel: String,
    pub pending: bool,
    pub personal_finance_category_id: i32, // Foreign key PersonalFinanceCategory
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct TransactionSQL {
    pub id: i32,
    pub account_id: i32, // Foreign key Account
    pub amount: f64,
    pub iso_currency_code: String,
    pub date: String,
    pub name: String,
    pub merchant_name: String,
    pub payment_channel: String,
    pub pending: bool,
    pub category: String,
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

#[enum_def] // => Generates PersonalFinanceCategoryIden NOTE: Move
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PersonalFinanceCategory {
    pub id: i32,
    pub primary: String,
    pub detailed: String,
}
