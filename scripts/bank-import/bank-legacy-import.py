#!/usr/bin/python3

import datetime
import pprint
import sqlite3
import requests
import os
import simplejson as json

def process_rows(rows, acc_id):
    for row in rows:
        print("input row: -------------------------\n")
        pprint.pprint(row)

        details = {
            'details_type': 'bank-legacy',
            'purpose': row[3],
            'entry_date': row[4],
            'storno_flag': row[5],
            'status': row[6],
            'swift_code': row[8],
            'reference': row[9] or '',
            'bank_reference': row[10] or '',
            'transaction_code': row[11],
        }
        
        details_json = json.dumps(details)
        debit_acc = ""
        credit_acc = ""

        amount_cents = row[2]
        if amount_cents<0:
            debit_acc = "assets:bank"
            amount_cents = -amount_cents
        else:
            credit_acc = "assets:bank"

        post_data = {
            'txn_id': row[0],
            'booking_date': row[1],
            'amount_cents': amount_cents,
            'currency': row[7],
            'details': details_json,
            'debit_account': debit_acc,
            'credit_account': credit_acc
        }

        if row[8]=="NMSC":
            pprint.pprint(post_data)
            r = requests.post('http://127.0.0.1:8080/bookings.json', json=post_data)
        else:
            print("skipped non-NMSC")

def main():
    acc_id = os.environ["BANK_ACC"]
    db_filename = "bank-"+str(acc_id)+".db"
    db = sqlite3.connect(db_filename)

    cur = db.cursor()
    cur.execute("SELECT id, date, amount_cents, details, entry_date, storno_flag, funds_code, currency_letter, swift_code, reference, bank_reference, transaction_code, seperator, source from transactions")
    rows = cur.fetchall()

    process_rows(rows, acc_id)
    db.close()
    
main()
