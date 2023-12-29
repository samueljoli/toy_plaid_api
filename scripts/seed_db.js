const { faker } = require('@faker-js/faker');
const knex = require('knex')
const fs = require('fs')
const { parse } = require('csv-parse');
const {
  log
} = console

const ISO_CURRENCY_CODE = "USD"

const parse_csv = async (path, transformer) => {
  const data = []

  const records = fs
    .createReadStream(path)
    .pipe(parse({
      columns: true,
    }))

  for await (const record of records) {
    data.push(transformer(record))
  }

  return data
}

const category_name_to_id_map = (records) => {
  const map = {}

  for (const record of records) {
    map[record.detailed] = record.id
  }

  return map
}

const seed_company = async (client) => {
  log("Seeding company...")

  return client('company').insert({
    name: 'Oxide',
    slug: 'oxide-computing',
  }).returning("id")
}

const seed_personal_finance_categories = async (client) => {
  log("Seeding personal finance categories...")

  const data = await parse_csv(`${__dirname}/personal_finance_categories.csv`, (record) => ({
    primary_category: record.primary,
    detailed: record.detailed,
  }))

  return client('personal_finance_category').insert(data).returning('*')
}

const seed_checkings_account = async (client, categories_map, company) => {
  log("Seeding checkings account...")

  let checking_transactions = []

  const [account] = await client('account').insert(
    {
      company_id: company.id,
      mask: '0000',
      name: 'Plaid Checking',
      official_name: 'Plaid Gold Checking',
      subtype: 'checking',
      type: 'depository'
    }
  ).returning('id')

  const data = await parse_csv(`${__dirname}/checkings_transactions.csv`, (record) => ({
    account_id: account.id,
    amount: record.amount,
    date: record.date,
    datetime: record.datetime,
    iso_currency_code: ISO_CURRENCY_CODE,
    merchant_name: record.merchant_name,
    name: record.name,
    payment_channel: record.payment_channel,
    pending: false,
    personal_finance_category_id: categories_map[record.personal_finance_category],
  }))

  await client('transaction').insert(data)
}

(async function() {
  const client = knex({
    client: 'pg',
    connection: {
      host: 'localhost',
      port: 5432,
      database: 'plaid',
      user: 'postgres',
      password: 'password',
    }
  })

  client.transaction(async function(trx) {
    const categories = await seed_personal_finance_categories(trx);

    const categories_map = category_name_to_id_map(categories)

    const [company] = await seed_company(trx);

    await seed_checkings_account(trx, categories_map, company);
  })
    .then(function() {
      log("Closing connection")

      process.exit(0)
    })
    .catch(function(error) {
      log("Uh oh", error)

      process.exit(1)
    });
}())
