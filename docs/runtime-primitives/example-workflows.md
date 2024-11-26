### Money

Boot Steps

```c

wget https://npci.org.in/upi/rtp/mod -o upi.rtp.mod

load_module("upi.rtp.mod");
mount("india.upi.rtp", "/udam/upi");
```

My Program

```c


void icici_account_transfer(char* from_upi_id, char* to_upi_id, uint32_t amount) {
    intent_id intent_from = intend(sprintf("/udam/upi/%s", from), O_DEBIT);
    intent_id intent_to = intend(sprintf("/udam/upi/%s", to), O_CREDIT);

    int exit_code = transfer(intent_from, intent_to, amount);

    if (exit_code == 0) {
        printf("Transfer successful\n");
    } else {
        printf("Transfer failed\n");
    }

    done(intent_from);
    done(intent_to);
}

```

## Securities

Boot Steps

```c

wget https://zerodha.nsdl.sec.mod -o zerodha.sec.mod

load_module("zerodha.sec.mod");
mount("india.zerodha.sec", "/udam/zerodha");


wget https://npci.org.in/upi/rtp/mod -o upi.rtp.mod

load_module("upi.rtp.mod");
mount("india.upi.rtp", "/udam/upi");
```

```c
void sell_securities(char* dp_id, char* exchange, char* isin, uint32_t quantity) {
    intent_id intent_from = intend(sprintf("/udam/zerodha/%s/%s", dp_id, isin), O_DEBIT);
    intent_id intent_to = intend(sprintf("/udam/zerodha/nsdl/%s/%s", exchange, isin), O_CREDIT);
    int exit_code = transfer(intent_from, intent_to, quantity);

    if (exit_code == 0) {
        printf("Transfer successful\n");
    } else {
        printf("Transfer failed\n");
    }

    done(intent_from);
    done(intent_to);
}

void buy_securities(char* dp_id, char* exchange, char* isin, uint32_t quantity) {
    intent_id intent_from = intend(sprintf("/udam/zerodha/nsdl/%s/%s", exchange, isin), O_DEBIT);
    intent_id intent_to = intend(sprintf("/udam/zerodha/%s/%s", dp_id, isin), O_CREDIT);
    int exit_code = transfer(intent_from, intent_to, quantity);

    if (exit_code == 0) {
        printf("Transfer successful\n");
    } else {
        printf("Transfer failed\n");
    }

    done(intent_from);
    done(intent_to);
}
```

### Fractionalized Assets

```c
wget https://bbmp.org.in/land/mod -o bbmp.land.mod

load_module("bbmp.land.mod");
mount("india.bbmp.land", "/udam/bbmp");
```

```c

void make_fractions(char* land_id) {
    intent_id intent_from = intend(sprintf("/udam/bbmp/%s", land_id), O_READ);
    intent_id intent_to_1 = intend(sprintf("/udam/bbmp/%s.%d", land_id, 1), O_CREATE);
    intent_id intent_to_2 = intend(sprintf("/udam/bbmp/%s.%d", land_id, 2), O_CREATE);



}

```
