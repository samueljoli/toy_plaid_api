CREATE TABLE account (
  id SERIAL PRIMARY KEY, 
  mask VARCHAR(255) NOT NULL, 
  name VARCHAR(255) NOT NULL, 
  official_name VARCHAR(255) NOT NULL, 
  subtype VARCHAR(255) NOT NULL, 
  type VARCHAR(255) NOT NULL
);
CREATE TABLE personal_finance_category (
  id SERIAL PRIMARY KEY, 
  primary_category VARCHAR(255) NOT NULL, 
  detailed VARCHAR(255) NOT NULL
);
CREATE TABLE transaction (
  id SERIAL PRIMARY KEY, 
  account_id INT REFERENCES account(id), 
  amount FLOAT NOT NULL, 
  iso_currency_code VARCHAR(255) NOT NULL, 
  date DATE NOT NULL, 
  name VARCHAR(255) NOT NULL, 
  merchant_name VARCHAR(255) NOT NULL, 
  payment_channel VARCHAR(255) NOT NULL, 
  pending BOOLEAN NOT NULL, 
  personal_finance_category_id INT REFERENCES personal_finance_category(id)
);
CREATE TABLE items (
  id SERIAL PRIMARY KEY, 
  consent_expiration_time TIMESTAMP, 
  error VARCHAR(255), 
  institution_id VARCHAR(255) NOT NULL, 
  update_type VARCHAR(255) NOT NULL, 
  webhook VARCHAR(255) NOT NULL
);

ALTER TABLE
  personal_finance_category
ADD
  CONSTRAINT chk_primary_category CHECK (
    primary_category IN (
      'INCOME', 'TRANSFER_IN', 'TRANSFER_OUT',
      'LOAN_PAYMENTS', 'BANK_FEES', 'ENTERTAINMENT',
      'FOOD_AND_DRINK', 'GENERAL_MERCHANDISE',
      'HOME_IMPROVEMENT', 'MEDICAL', 'PERSONAL_CARE',
      'GENERAL_SERVICES', 'GOVERNMENT_AND_NON_PROFIT',
      'TRANSPORTATION', 'TRAVEL', 'RENT_AND_UTILITIES'
    )
  );
ALTER TABLE
  personal_finance_category
ADD
  CONSTRAINT chk_detailed CHECK (
    detailed IN (
      'INCOME_WAGES', 'INCOME_OTHER_INCOME',
      'TRANSFER_IN_CASH_ADVANCES_AND_LOANS', 'TRANSFER_IN_DEPOSIT',
      'TRANSFER_IN_SAVINGS', 'TRANSFER_IN_ACCOUNT_TRANSFER',
      'TRANSFER_IN_OTHER_TRANSFER_IN', 'TRANSFER_OUT_INVESTMENT_AND_RETIREMENT_FUNDS',
      'TRANSFER_OUT_SAVINGS', 'TRANSFER_OUT_WITHDRAWAL',
      'TRANSFER_OUT_ACCOUNT_TRANSFER', 'TRANSFER_OUT_OTHER_TRANSFER_OUT',
      'ENTERTAINMENT_OTHER_ENTERTAINMENT', 'FOOD_AND_DRINK_BEER_WINE_AND_LIQUOR',
      'FOOD_AND_DRINK_COFFEE', 'FOOD_AND_DRINK_FAST_FOOD',
      'FOOD_AND_DRINK_GROCERIES', 'FOOD_AND_DRINK_RESTAURANT',
      'GENERAL_MERCHANDISE_BOOKSTORES_AND_NEWSSTANDS',
      'GENERAL_MERCHANDISE_CLOTHING_AND_ACCESSORIES',
      'GENERAL_MERCHANDISE_CONVENIENCE_STORES',
      'GENERAL_MERCHANDISE_DEPARTMENT_STORES',
      'GENERAL_MERCHANDISE_DISCOUNT_STORES', 'GENERAL_MERCHANDISE_ELECTRONICS',
      'GENERAL_MERCHANDISE_GIFTS_AND_NOVELTIES',
      'GENERAL_MERCHANDISE_OFFICE_SUPPLIES',
      'GENERAL_MERCHANDISE_ONLINE_MARKETPLACES',
      'GENERAL_MERCHANDISE_PET_SUPPLIES', 'GENERAL_MERCHANDISE_SPORTING_GOODS',
      'GENERAL_MERCHANDISE_SUPERSTORES', 'GENERAL_MERCHANDISE_TOBACCO_AND_VAPE',
      'GENERAL_MERCHANDISE_OTHER_GENERAL_MERCHANDISE',
      'GENERAL_SERVICES_ACCOUNTING_AND_FINANCIAL_PLANNING',
      'GENERAL_SERVICES_AUTOMOTIVE', 'GENERAL_SERVICES_CHILDCARE',
      'GENERAL_SERVICES_CONSULTING_AND_LEGAL', 'GENERAL_SERVICES_EDUCATION',
      'GENERAL_SERVICES_INSURANCE', 'GENERAL_SERVICES_POSTAGE_AND_SHIPPING',
      'GENERAL_SERVICES_STORAGE', 'GENERAL_SERVICES_OTHER_GENERAL_SERVICES',
      'GOVERNMENT_AND_NON_PROFIT_DONATIONS',
      'GOVERNMENT_AND_NON_PROFIT_GOVERNMENT_DEPARTMENTS_AND_AGENCIES',
      'GOVERNMENT_AND_NON_PROFIT_TAX_PAYMENT',
      'GOVERNMENT_AND_NON_PROFIT_OTHER_GOVERNMENT_AND_NON_PROFIT',
      'TRANSPORTATION_TAXIS_AND_RIDE_SHARES', 'TRAVEL_FLIGHTS',
      'RENT_AND_UTILITIES_GAS_AND_ELECTRICITY',
      'RENT_AND_UTILITIES_INTERNET_AND_CABLE', 'RENT_AND_UTILITIES_RENT',
      'RENT_AND_UTILITIES_TELEPHONE'
    )
  );
