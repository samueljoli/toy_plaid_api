CREATE TABLE accounts (
  id SERIAL PRIMARY KEY, 
  mask VARCHAR(255) NOT NULL, 
  name VARCHAR(255) NOT NULL, 
  official_name VARCHAR(255) NOT NULL, 
  subtype VARCHAR(255) NOT NULL, 
  type VARCHAR(255) NOT NULL
);
CREATE TABLE locations (
  id SERIAL PRIMARY KEY, 
  address VARCHAR(255), 
  city VARCHAR(255), 
  region VARCHAR(255), 
  postal_code VARCHAR(255), 
  country VARCHAR(255), 
  lat FLOAT, 
  lon FLOAT, 
  store_number VARCHAR(255)
);
CREATE TABLE payment_meta (
  id SERIAL PRIMARY KEY, 
  by_order_of VARCHAR(255), 
  payee VARCHAR(255), 
  payer VARCHAR(255), 
  payment_method VARCHAR(255), 
  payment_processor VARCHAR(255), 
  ppd_id VARCHAR(255), 
  reason VARCHAR(255), 
  reference_number VARCHAR(255)
);
CREATE TABLE personal_finance_categories (
  id SERIAL PRIMARY KEY, 
  primary_category VARCHAR(255) NOT NULL, 
  detailed VARCHAR(255) NOT NULL, 
  confidence_level VARCHAR(255) NOT NULL
);
CREATE TABLE transactions (
  id SERIAL PRIMARY KEY, 
  account_id INT REFERENCES accounts(id), 
  amount FLOAT NOT NULL, 
  iso_currency_code VARCHAR(255) NOT NULL, 
  unofficial_currency_code VARCHAR(255), 
  check_number VARCHAR(255), 
  date DATE NOT NULL, 
  datetime TIMESTAMP NOT NULL, 
  authorized_date DATE NOT NULL, 
  authorized_datetime TIMESTAMP NOT NULL, 
  location_id INT REFERENCES locations(id), 
  name VARCHAR(255) NOT NULL, 
  merchant_name VARCHAR(255) NOT NULL, 
  merchant_entity_id VARCHAR(255) NOT NULL, 
  logo_url VARCHAR(255) NOT NULL, 
  website VARCHAR(255) NOT NULL, 
  payment_meta_id INT REFERENCES payment_meta(id), 
  payment_channel VARCHAR(255) NOT NULL, 
  pending BOOLEAN NOT NULL, 
  pending_transaction_id VARCHAR(255), 
  personal_finance_category_id INT REFERENCES personal_finance_categories(id), 
  personal_finance_category_icon_url VARCHAR(255) NOT NULL, 
  transaction_code VARCHAR(255)
);
CREATE TABLE items (
  id SERIAL PRIMARY KEY, 
  consent_expiration_time TIMESTAMP, 
  error VARCHAR(255), 
  institution_id VARCHAR(255) NOT NULL, 
  update_type VARCHAR(255) NOT NULL, 
  webhook VARCHAR(255) NOT NULL
);
-- TransactionPersonalFinanceCategory Join Table
CREATE TABLE transaction_personal_finance_categories (
  transaction_id INT REFERENCES transactions(id), 
  personal_finance_category_id INT REFERENCES personal_finance_categories(id), 
  PRIMARY KEY (
    transaction_id, personal_finance_category_id
  )
);
-- Enum Tables or Check Constraints (example for one Enum)
-- For simplicity, using check constraints here
ALTER TABLE 
  personal_finance_categories 
ADD 
  CONSTRAINT chk_primary_category CHECK (
    primary_category IN (
      'Income', 'TransferIn', 'TransferOut', 
      'LoanPayments', 'BankFees', 'Entertainment', 
      'FoodAndDrink', 'GeneralMerchandise', 
      'HomeImprovement', 'Medical', 'PersonalCare', 
      'GeneralServices', 'GovernmentAndNonProfit', 
      'Transportation', 'Travel', 'RentAndUtilities'
    )
  );
ALTER TABLE 
  personal_finance_categories 
ADD 
  CONSTRAINT chk_detailed CHECK (
    detailed IN (
      'IncomeDividends', 'IncomeInterestEarned', 
      'IncomeRetirementPension', 'IncomeTaxRefund', 
      'IncomeUnemployment', 'IncomeWages', 
      'IncomeOtherIncome', 'TransferInCashAdvancesAndLoans', 
      'TransferInDeposit', 'TransferInInvestmentAndRetirementFunds', 
      'TransferInSavings', 'TransferInAccountTransfer', 
      'TransferInOtherTransferIn', 'TransferOutInvestmentAndRetirementFunds', 
      'TransferOutSavings', 'TransferOutWithdrawal', 
      'TransferOutAccountTransfer', 'TransferOutOtherTransferOut', 
      'LoanPaymentsCarPayment', 'LoanPaymentsCreditCardPayment', 
      'LoanPaymentsPersonalLoanPayment', 
      'LoanPaymentsMortgagePayment', 
      'LoanPaymentsStudentLoanPayment', 
      'LoanPaymentsOtherPayment', 'BankFeesAtmFees', 
      'BankFeesForeignTransaction'
    )
  );

