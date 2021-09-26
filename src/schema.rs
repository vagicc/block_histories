table! {
    admins (id) {
        id -> Unsigned<Integer>,
        username -> Varchar,
        password -> Varchar,
        salt -> Nullable<Char>,
        email -> Nullable<Varchar>,
        mobile -> Nullable<Char>,
        role -> Nullable<Unsigned<Smallint>>,
        status -> Nullable<Bigint>,
        create_time -> Datetime,
        last_login -> Nullable<Bigint>,
    }
}

table! {
    bzz_node_info (id) {
        id -> Bigint,
        server_id -> Unsigned<Integer>,
        user_id -> Bigint,
        ip -> Nullable<Varchar>,
        api_port -> Nullable<Integer>,
        ethereum_address -> Nullable<Varchar>,
        peers -> Nullable<Integer>,
        chequebook_address -> Nullable<Varchar>,
        cheque_num -> Nullable<Integer>,
        uncashed_num -> Nullable<Integer>,
        state -> Nullable<Smallint>,
        version -> Nullable<Varchar>,
        update_timestamp -> Nullable<Bigint>,
        create_time -> Nullable<Datetime>,
    }
}

table! {
    bzz_node_info_new (tid) {
        tid -> Bigint,
        id -> Unsigned<Integer>,
        server_id -> Unsigned<Integer>,
        user_id -> Bigint,
        ip -> Nullable<Varchar>,
        api_port -> Nullable<Integer>,
        ethereum_address -> Nullable<Varchar>,
        peers -> Nullable<Integer>,
        chequebook_address -> Nullable<Varchar>,
        cheque_num -> Nullable<Integer>,
        uncashed_num -> Nullable<Integer>,
        state -> Nullable<Smallint>,
        version -> Nullable<Varchar>,
        update_timestamp -> Nullable<Bigint>,
        update_time -> Nullable<Datetime>,
    }
}

table! {
    bzz_server_hosting (id) {
        id -> Bigint,
        server_name -> Varchar,
        computer_room -> Varchar,
        cpuinfo -> Nullable<Varchar>,
        ip1 -> Nullable<Varchar>,
        ip2 -> Nullable<Varchar>,
        state -> Bool,
        admin_id -> Bigint,
        user_id -> Bigint,
        create_time -> Nullable<Datetime>,
    }
}

table! {
    bzz_server_system (id) {
        id -> Bigint,
        server_id -> Unsigned<Integer>,
        user_id -> Bigint,
        ip -> Nullable<Varchar>,
        cpu_nums -> Bool,
        cpu_used_rate -> Nullable<Decimal>,
        memory_used_rate -> Nullable<Decimal>,
        memory_size_capacity -> Nullable<Bigint>,
        broadhand_in -> Nullable<Varchar>,
        broadhand_out -> Nullable<Varchar>,
        node_number -> Integer,
        update_timestamp -> Nullable<Bigint>,
        create_time -> Datetime,
    }
}

table! {
    bzz_server_system_new (tid) {
        tid -> Bigint,
        id -> Unsigned<Integer>,
        server_id -> Unsigned<Integer>,
        user_id -> Bigint,
        ip -> Nullable<Varchar>,
        cpu_nums -> Bool,
        cpu_used_rate -> Nullable<Decimal>,
        memory_used_rate -> Nullable<Decimal>,
        memory_size_capacity -> Nullable<Bigint>,
        broadhand_in -> Nullable<Varchar>,
        broadhand_out -> Nullable<Varchar>,
        node_number -> Integer,
        update_timestamp -> Nullable<Bigint>,
        update_time -> Datetime,
    }
}

table! {
    carousel (id) {
        id -> Unsigned<Bigint>,
        link -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        path -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        extension -> Nullable<Varchar>,
        show -> Nullable<Integer>,
        order_by -> Nullable<Integer>,
        uid -> Nullable<Integer>,
        create_time -> Nullable<Bigint>,
    }
}

table! {
    chia_block_histories (id) {
        id -> Bigint,
        coin_id -> Nullable<Varchar>,
        puzzle_hash -> Nullable<Varchar>,
        parent_coin_info -> Nullable<Varchar>,
        amount -> Nullable<Bigint>,
        coinbase -> Nullable<Varchar>,
        confirmed_block_index -> Nullable<Bigint>,
        spent -> Nullable<Varchar>,
        spent_block_index -> Nullable<Bigint>,
        timestamp -> Nullable<Bigint>,
        block_header_hash -> Nullable<Varchar>,
        block_height -> Nullable<Bigint>,
        pool_id -> Bigint,
        pool_name -> Nullable<Varchar>,
        pool_type -> Nullable<Varchar>,
        groover_id -> Bigint,
        pool_order -> Nullable<Integer>,
        groover -> Nullable<Varchar>,
        wallet_id -> Nullable<Varchar>,
        wallet_address -> Nullable<Varchar>,
        status -> Tinyint,
        modify -> Datetime,
        create_time -> Datetime,
    }
}

table! {
    chia_server_system (id) {
        id -> Bigint,
        server_id -> Unsigned<Integer>,
        customer_id -> Bigint,
        ip -> Nullable<Varchar>,
        cpu_nums -> Bool,
        cpu_used_rate -> Nullable<Decimal>,
        memory_used_rate -> Nullable<Decimal>,
        memory_size_capacity -> Nullable<Bigint>,
        broadhand_in -> Nullable<Varchar>,
        broadhand_out -> Nullable<Varchar>,
        the_number_of_files -> Varchar,
        file_size -> Varchar,
        update_timestamp -> Nullable<Bigint>,
        create_time -> Datetime,
    }
}

table! {
    chia_server_system_new (tid) {
        tid -> Bigint,
        id -> Unsigned<Integer>,
        server_id -> Unsigned<Integer>,
        customer_id -> Bigint,
        ip -> Nullable<Varchar>,
        cpu_nums -> Bool,
        cpu_used_rate -> Nullable<Decimal>,
        memory_used_rate -> Nullable<Decimal>,
        memory_size_capacity -> Nullable<Bigint>,
        broadhand_in -> Nullable<Varchar>,
        broadhand_out -> Nullable<Varchar>,
        the_number_of_files -> Varchar,
        file_size -> Varchar,
        update_timestamp -> Nullable<Bigint>,
        update_time -> Datetime,
    }
}

table! {
    coin_pool (id) {
        id -> Bigint,
        pool_name -> Nullable<Varchar>,
        pool_type -> Nullable<Varchar>,
        pool_address -> Nullable<Varchar>,
        wallet_id -> Nullable<Varchar>,
        wallet_address -> Nullable<Varchar>,
        spaces_count -> Bigint,
        allocated -> Bigint,
        coin -> Bigint,
        coin_cllot -> Bigint,
        block -> Bigint,
        computer_room -> Nullable<Varchar>,
        server_nums -> Nullable<Bool>,
        admin_id -> Bigint,
        create_time -> Datetime,
        allot -> Tinyint,
    }
}

table! {
    coin_pool_allot (id) {
        id -> Bigint,
        ratio_id -> Bigint,
        daily -> Date,
        pool_id -> Nullable<Bigint>,
        pool_name -> Nullable<Varchar>,
        pool_type -> Nullable<Varchar>,
        pool_wallet_address -> Nullable<Varchar>,
        spaces_count -> Bigint,
        customer_id -> Bigint,
        username -> Nullable<Varchar>,
        customer_wallet_address -> Nullable<Varchar>,
        spaces -> Bigint,
        ratio -> Nullable<Decimal>,
        coin_number -> Nullable<Decimal>,
        pool_coin -> Nullable<Decimal>,
        amount -> Nullable<Decimal>,
        block_height -> Nullable<Bigint>,
        block_time -> Nullable<Bigint>,
        status -> Tinyint,
        admin_id -> Bigint,
        create_time -> Datetime,
        tx_id -> Nullable<Varchar>,
        hash -> Nullable<Varchar>,
        last_time -> Nullable<Datetime>,
    }
}

table! {
    coin_pool_groover (id) {
        id -> Bigint,
        pool_id -> Bigint,
        pool_name -> Nullable<Varchar>,
        pool_type -> Nullable<Varchar>,
        pool_order -> Nullable<Integer>,
        groover -> Nullable<Varchar>,
        wallet_id -> Nullable<Varchar>,
        wallet_address -> Nullable<Varchar>,
        mnemonic -> Nullable<Text>,
        passwd -> Nullable<Varchar>,
        admin_id -> Bigint,
        modify -> Datetime,
        create_time -> Datetime,
    }
}

table! {
    coin_pool_ratio (id) {
        id -> Bigint,
        pool_id -> Nullable<Bigint>,
        pool_name -> Nullable<Varchar>,
        pool_type -> Nullable<Varchar>,
        pool_wallet_address -> Nullable<Varchar>,
        customer_id -> Bigint,
        username -> Nullable<Varchar>,
        customer_wallet_address -> Nullable<Varchar>,
        spaces -> Bigint,
        ratio -> Nullable<Decimal>,
        admin_id -> Bigint,
        modify -> Datetime,
    }
}

table! {
    coin_pool_ratio_log (id) {
        id -> Bigint,
        pool_id -> Bigint,
        pool_name -> Nullable<Varchar>,
        customer_id -> Bigint,
        spaces -> Bigint,
        admin_id -> Bigint,
        create_time -> Datetime,
    }
}

table! {
    coin_top_up_withdrawal (id) {
        id -> Bigint,
        coin_wallet_sn -> Varchar,
        customer_id -> Bigint,
        create_time -> Datetime,
        modify_time -> Datetime,
        coin_type -> Nullable<Varchar>,
        water_type -> Bool,
        charge_rate -> Nullable<Varchar>,
        charge -> Nullable<Decimal>,
        amount -> Nullable<Decimal>,
        balance -> Nullable<Decimal>,
        freeze -> Nullable<Decimal>,
        available -> Nullable<Decimal>,
        is_add -> Bool,
        origin -> Nullable<Varchar>,
        water -> Nullable<Varchar>,
        tx_id -> Nullable<Varchar>,
        status -> Tinyint,
        admin_id -> Nullable<Bigint>,
    }
}

table! {
    coin_type (id) {
        id -> Bigint,
        coin_name -> Nullable<Varchar>,
        coin -> Varchar,
        decimals -> Tinyint,
        coin_rate -> Nullable<Decimal>,
        allot_ratio -> Nullable<Decimal>,
        coin_rate_two -> Nullable<Unsigned<Integer>>,
        rate_option -> Tinyint,
        withdrawal_limit -> Nullable<Unsigned<Integer>>,
        onw_withdrawal_limit -> Nullable<Unsigned<Integer>>,
        minimum -> Nullable<Unsigned<Integer>>,
        coin_switch -> Bool,
        collects_switch -> Bool,
        collects_wallet_address -> Nullable<Varchar>,
        update_time -> Datetime,
    }
}

table! {
    coin_wallet (id) {
        id -> Bigint,
        customer_id -> Bigint,
        modify -> Datetime,
        balance_version -> Bigint,
        btc -> Nullable<Decimal>,
        btc_freeze -> Nullable<Decimal>,
        btc_available -> Nullable<Decimal>,
        eth -> Nullable<Decimal>,
        eth_freeze -> Nullable<Decimal>,
        eth_available -> Nullable<Decimal>,
        usdt -> Nullable<Decimal>,
        usdt_freeze -> Nullable<Decimal>,
        usdt_available -> Nullable<Decimal>,
        mass -> Nullable<Decimal>,
        mass_freeze -> Nullable<Decimal>,
        mass_available -> Nullable<Decimal>,
        pha -> Nullable<Decimal>,
        pha_freeze -> Nullable<Decimal>,
        pha_available -> Nullable<Decimal>,
    }
}

table! {
    coin_wallet_chia (id) {
        id -> Bigint,
        customer_id -> Bigint,
        modify -> Datetime,
        balance_version -> Bigint,
        chia -> Nullable<Decimal>,
        chia_freeze -> Nullable<Decimal>,
        chia_available -> Nullable<Decimal>,
    }
}

table! {
    coin_wallet_water (id) {
        id -> Bigint,
        coin_wallet_sn -> Varchar,
        customer_id -> Bigint,
        create_time -> Datetime,
        modify_time -> Datetime,
        balance_version -> Bigint,
        coin_type -> Nullable<Varchar>,
        transaction_type -> Tinyint,
        amount -> Nullable<Decimal>,
        balance -> Nullable<Decimal>,
        freeze -> Nullable<Decimal>,
        available -> Nullable<Decimal>,
        is_add -> Bool,
        desc -> Nullable<Varchar>,
        origin -> Nullable<Varchar>,
        water -> Nullable<Varchar>,
        status -> Tinyint,
        admin_id -> Nullable<Bigint>,
    }
}

table! {
    cru_block_histories (id) {
        id -> Bigint,
        extrinsic_index -> Varchar,
        block_timestamp -> Nullable<Bigint>,
        generated_at -> Nullable<Bigint>,
        signature -> Nullable<Varchar>,
        extrinsic_hash -> Varchar,
        block_num -> Nullable<Bigint>,
        extrinsic_idx -> Nullable<Integer>,
        module_id -> Nullable<Varchar>,
        event_id -> Nullable<Varchar>,
        event_idx -> Nullable<Integer>,
        account_id -> Nullable<Varchar>,
        amount -> Nullable<Bigint>,
        finalized -> Nullable<Varchar>,
        pool_id -> Bigint,
        pool_name -> Nullable<Varchar>,
        pool_type -> Nullable<Varchar>,
        groover_id -> Bigint,
        pool_order -> Nullable<Integer>,
        groover -> Nullable<Varchar>,
        wallet_id -> Nullable<Varchar>,
        wallet_address -> Nullable<Varchar>,
        status -> Tinyint,
        modify -> Datetime,
        create_time -> Datetime,
    }
}

table! {
    customer (id) {
        id -> Integer,
        customer_id -> Unsigned<Integer>,
        invitation_code -> Char,
        email -> Nullable<Varchar>,
        mobile -> Char,
        username -> Varchar,
        password -> Varchar,
        salt -> Nullable<Char>,
        create_time -> Datetime,
        last_login -> Nullable<Bigint>,
    }
}

table! {
    find_block_histories (id) {
        id -> Bigint,
        coin_type -> Nullable<Varchar>,
        block_num -> Nullable<Bigint>,
        cycles_num -> Nullable<Bigint>,
        status -> Tinyint,
        optimism -> Bigint,
        modify -> Datetime,
        create_time -> Datetime,
    }
}

table! {
    mass_block_abnormal (id) {
        id -> Bigint,
        server_id -> Unsigned<Integer>,
        customer_id -> Bigint,
        ip -> Nullable<Varchar>,
        local_best_height -> Bigint,
        update_timestamp -> Nullable<Bigint>,
        update_time -> Datetime,
    }
}

table! {
    mass_block_histories (id) {
        id -> Bigint,
        tx_id -> Nullable<Varchar>,
        block_height -> Bigint,
        block_time -> Nullable<Bigint>,
        amount -> Nullable<Decimal>,
        from_addresses -> Nullable<Varchar>,
        wallet_address -> Nullable<Varchar>,
        pool_id -> Bigint,
        pool_name -> Nullable<Varchar>,
        pool_type -> Nullable<Varchar>,
        pool_address -> Nullable<Varchar>,
        groover_id -> Nullable<Bigint>,
        groover_name -> Nullable<Varchar>,
        status -> Tinyint,
        modify -> Datetime,
        create_time -> Datetime,
    }
}

table! {
    mass_server_system (id) {
        id -> Bigint,
        server_id -> Unsigned<Integer>,
        customer_id -> Bigint,
        ip -> Nullable<Varchar>,
        plots -> Bigint,
        spaces -> Bigint,
        local_best_height -> Bigint,
        peer_count -> Bigint,
        state -> Bool,
        mining -> Bool,
        syncing -> Bool,
        version -> Nullable<Varchar>,
        update_timestamp -> Nullable<Bigint>,
        update_time -> Datetime,
    }
}

table! {
    mass_server_system_new (tid) {
        tid -> Bigint,
        id -> Unsigned<Integer>,
        server_id -> Unsigned<Integer>,
        customer_id -> Bigint,
        ip -> Nullable<Varchar>,
        plots -> Bigint,
        spaces -> Bigint,
        local_best_height -> Bigint,
        peer_count -> Bigint,
        state -> Bool,
        mining -> Bool,
        syncing -> Bool,
        version -> Nullable<Varchar>,
        update_timestamp -> Nullable<Bigint>,
        update_time -> Datetime,
    }
}

table! {
    menus (id) {
        id -> Unsigned<Tinyint>,
        order_by -> Unsigned<Tinyint>,
        class -> Varchar,
        method -> Varchar,
        name -> Varchar,
        level -> Nullable<Unsigned<Tinyint>>,
        parent -> Nullable<Unsigned<Tinyint>>,
        icon -> Nullable<Varchar>,
        department -> Nullable<Varchar>,
        is_show -> Bool,
    }
}

table! {
    oauth_access_tokens (access_token) {
        access_token -> Varchar,
        client_id -> Varchar,
        user_id -> Nullable<Varchar>,
        expires -> Timestamp,
        scope -> Nullable<Varchar>,
    }
}

table! {
    oauth_authorization_codes (authorization_code) {
        authorization_code -> Varchar,
        client_id -> Varchar,
        user_id -> Nullable<Varchar>,
        redirect_uri -> Nullable<Varchar>,
        expires -> Timestamp,
        scope -> Nullable<Varchar>,
        id_token -> Nullable<Varchar>,
    }
}

table! {
    oauth_clients (client_id) {
        client_id -> Varchar,
        client_secret -> Nullable<Varchar>,
        redirect_uri -> Nullable<Varchar>,
        grant_types -> Nullable<Varchar>,
        scope -> Nullable<Varchar>,
        user_id -> Nullable<Varchar>,
    }
}

table! {
    oauth_jwt (client_id) {
        client_id -> Varchar,
        subject -> Nullable<Varchar>,
        public_key -> Varchar,
    }
}

table! {
    oauth_refresh_tokens (refresh_token) {
        refresh_token -> Varchar,
        client_id -> Varchar,
        user_id -> Nullable<Varchar>,
        expires -> Timestamp,
        scope -> Nullable<Varchar>,
    }
}

table! {
    oauth_scopes (scope) {
        scope -> Varchar,
        is_default -> Nullable<Bool>,
    }
}

table! {
    oauth_users (username) {
        username -> Varchar,
        password -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        email_verified -> Nullable<Bool>,
        scope -> Nullable<Varchar>,
    }
}

table! {
    pha_daily_coin (id) {
        id -> Bigint,
        user_id -> Bigint,
        server_id -> Unsigned<Integer>,
        server_name -> Nullable<Varchar>,
        daily -> Date,
        start_bonus -> Nullable<Decimal>,
        last_bonus -> Nullable<Decimal>,
        post_updated -> Nullable<Bigint>,
        create_time -> Datetime,
        last_time -> Nullable<Datetime>,
    }
}

table! {
    pha_node_info (id) {
        id -> Bigint,
        server_id -> Unsigned<Integer>,
        user_id -> Bigint,
        ip -> Nullable<Varchar>,
        api_port -> Nullable<Integer>,
        service -> Nullable<Varchar>,
        peers -> Nullable<Integer>,
        chequebook_address -> Nullable<Varchar>,
        cheque_num -> Nullable<Integer>,
        uncashed_num -> Nullable<Integer>,
        state -> Nullable<Smallint>,
        version -> Nullable<Varchar>,
        update_timestamp -> Nullable<Bigint>,
        create_time -> Nullable<Datetime>,
    }
}

table! {
    pha_node_info_new (tid) {
        tid -> Bigint,
        id -> Unsigned<Integer>,
        server_id -> Unsigned<Integer>,
        user_id -> Bigint,
        ip -> Nullable<Varchar>,
        api_port -> Nullable<Integer>,
        service -> Nullable<Varchar>,
        peers -> Nullable<Integer>,
        chequebook_address -> Nullable<Varchar>,
        cheque_num -> Nullable<Integer>,
        uncashed_num -> Nullable<Integer>,
        state -> Nullable<Smallint>,
        version -> Nullable<Varchar>,
        update_timestamp -> Nullable<Bigint>,
        update_time -> Nullable<Datetime>,
    }
}

table! {
    pha_server_coin (id) {
        id -> Bigint,
        server_id -> Unsigned<Integer>,
        server_name -> Nullable<Varchar>,
        ip_address -> Nullable<Varchar>,
        machine_id -> Nullable<Varchar>,
        pubkey -> Nullable<Varchar>,
        wallet_address -> Nullable<Varchar>,
        confidence_level -> Bool,
        overall_score -> Bool,
        cpu_nums -> Bool,
        features -> Bool,
        state -> Nullable<Varchar>,
        last_updated -> Nullable<Bigint>,
        runtime_version -> Integer,
        bonus -> Nullable<Decimal>,
        post_time -> Nullable<Bigint>,
        create_time -> Datetime,
    }
}

table! {
    pha_server_coin_new (id) {
        id -> Bigint,
        server_id -> Unsigned<Integer>,
        server_name -> Nullable<Varchar>,
        user_id -> Integer,
        ip_address -> Nullable<Varchar>,
        machine_id -> Nullable<Varchar>,
        pubkey -> Nullable<Varchar>,
        wallet_address -> Nullable<Varchar>,
        confidence_level -> Bool,
        overall_score -> Bool,
        machine_score -> Nullable<Varchar>,
        cpu_nums -> Bool,
        features -> Bool,
        state -> Nullable<Varchar>,
        last_updated -> Nullable<Bigint>,
        runtime_version -> Integer,
        bonus -> Nullable<Decimal>,
        post_time -> Nullable<Bigint>,
        create_time -> Datetime,
    }
}

table! {
    pha_server_hosting (id) {
        id -> Bigint,
        server_name -> Varchar,
        computer_room -> Varchar,
        cpuinfo -> Nullable<Varchar>,
        ip1 -> Nullable<Varchar>,
        ip2 -> Nullable<Varchar>,
        state -> Bool,
        admin_id -> Bigint,
        user_id -> Bigint,
        create_time -> Nullable<Datetime>,
    }
}

table! {
    pha_server_system (id) {
        id -> Bigint,
        server_id -> Unsigned<Integer>,
        user_id -> Bigint,
        ip -> Nullable<Varchar>,
        cpu_nums -> Bool,
        wallet_address -> Nullable<Varchar>,
        confidence_level -> Bool,
        overall_score -> Bool,
        features1 -> Bool,
        state -> Nullable<Varchar>,
        last_updated -> Nullable<Bigint>,
        runtime_version -> Integer,
        bonus -> Nullable<Decimal>,
        post_time -> Nullable<Bigint>,
        cpu_used_rate -> Nullable<Decimal>,
        memory_used_rate -> Nullable<Decimal>,
        memory_size_capacity -> Nullable<Bigint>,
        broadhand_in -> Nullable<Varchar>,
        broadhand_out -> Nullable<Varchar>,
        node_number -> Integer,
        update_timestamp -> Nullable<Bigint>,
        create_time -> Datetime,
    }
}

table! {
    pha_server_system_new (tid) {
        tid -> Bigint,
        id -> Unsigned<Integer>,
        server_id -> Unsigned<Integer>,
        user_id -> Bigint,
        ip -> Nullable<Varchar>,
        cpu_nums -> Bool,
        cpu_used_rate -> Nullable<Decimal>,
        memory_used_rate -> Nullable<Decimal>,
        memory_size_capacity -> Nullable<Bigint>,
        broadhand_in -> Nullable<Varchar>,
        broadhand_out -> Nullable<Varchar>,
        node_number -> Integer,
        update_timestamp -> Nullable<Bigint>,
        update_time -> Datetime,
    }
}

table! {
    record (id) {
        id -> Unsigned<Integer>,
        table_id -> Unsigned<Integer>,
        table_name -> Varchar,
        user_id -> Unsigned<Integer>,
        username -> Varchar,
        action -> Varchar,
        ip -> Varchar,
        record_time -> Datetime,
    }
}

table! {
    rights (right_id) {
        right_id -> Unsigned<Tinyint>,
        right_name -> Nullable<Varchar>,
        right_class -> Nullable<Varchar>,
        right_method -> Nullable<Varchar>,
        right_detail -> Nullable<Varchar>,
    }
}

table! {
    roles (id) {
        id -> Integer,
        name -> Varchar,
        rights -> Nullable<Varchar>,
        default -> Nullable<Varchar>,
    }
}

table! {
    server_hosting (id) {
        id -> Bigint,
        server_name -> Varchar,
        computer_room -> Nullable<Varchar>,
        cpuinfo -> Nullable<Varchar>,
        cpu_nums -> Nullable<Bool>,
        hard_disk -> Nullable<Varchar>,
        memory -> Nullable<Varchar>,
        radiator -> Nullable<Varchar>,
        power -> Nullable<Varchar>,
        ip1 -> Nullable<Varchar>,
        ip2 -> Nullable<Varchar>,
        state -> Bool,
        notes -> Nullable<Text>,
        coin_type -> Nullable<Varchar>,
        admin_id -> Nullable<Bigint>,
        customer_id -> Unsigned<Integer>,
        create_time -> Nullable<Datetime>,
    }
}

table! {
    sms_verification (id) {
        id -> Unsigned<Bigint>,
        mobile -> Char,
        sms_code -> Char,
        sms_type -> Bool,
        state -> Bool,
        ip -> Nullable<Varchar>,
        timestamp -> Nullable<Bigint>,
        update_time -> Datetime,
    }
}

table! {
    xfx_block_histories (id) {
        id -> Bigint,
        tx_id -> Nullable<Varchar>,
        block_height -> Bigint,
        height_spent -> Nullable<Varchar>,
        block_time -> Nullable<Bigint>,
        time_str -> Nullable<Varchar>,
        block_type -> Nullable<Varchar>,
        amount -> Nullable<Decimal>,
        amount_text -> Nullable<Varchar>,
        amount_mojo -> Nullable<Varchar>,
        pool_id -> Bigint,
        pool_name -> Nullable<Varchar>,
        pool_type -> Nullable<Varchar>,
        groover_id -> Bigint,
        pool_order -> Nullable<Integer>,
        groover -> Nullable<Varchar>,
        wallet_id -> Nullable<Varchar>,
        wallet_address -> Nullable<Varchar>,
        status -> Tinyint,
        modify -> Datetime,
        create_time -> Datetime,
    }
}

table! {
    xfx_server_system (id) {
        id -> Bigint,
        server_id -> Unsigned<Integer>,
        customer_id -> Bigint,
        ip -> Nullable<Varchar>,
        plots -> Bigint,
        spaces -> Bigint,
        local_best_height -> Bigint,
        peer_count -> Bigint,
        state -> Bool,
        state_str -> Nullable<Varchar>,
        mining -> Bool,
        syncing -> Bool,
        version -> Nullable<Varchar>,
        update_timestamp -> Nullable<Bigint>,
        update_time -> Datetime,
    }
}

table! {
    xfx_server_system_new (tid) {
        tid -> Bigint,
        id -> Unsigned<Integer>,
        server_id -> Unsigned<Integer>,
        customer_id -> Bigint,
        ip -> Nullable<Varchar>,
        plots -> Bigint,
        spaces -> Bigint,
        local_best_height -> Bigint,
        peer_count -> Bigint,
        state -> Bool,
        state_str -> Nullable<Varchar>,
        mining -> Bool,
        syncing -> Bool,
        version -> Nullable<Varchar>,
        update_timestamp -> Nullable<Bigint>,
        update_time -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(
    admins,
    bzz_node_info,
    bzz_node_info_new,
    bzz_server_hosting,
    bzz_server_system,
    bzz_server_system_new,
    carousel,
    chia_block_histories,
    chia_server_system,
    chia_server_system_new,
    coin_pool,
    coin_pool_allot,
    coin_pool_groover,
    coin_pool_ratio,
    coin_pool_ratio_log,
    coin_top_up_withdrawal,
    coin_type,
    coin_wallet,
    coin_wallet_chia,
    coin_wallet_water,
    cru_block_histories,
    customer,
    find_block_histories,
    mass_block_abnormal,
    mass_block_histories,
    mass_server_system,
    mass_server_system_new,
    menus,
    oauth_access_tokens,
    oauth_authorization_codes,
    oauth_clients,
    oauth_jwt,
    oauth_refresh_tokens,
    oauth_scopes,
    oauth_users,
    pha_daily_coin,
    pha_node_info,
    pha_node_info_new,
    pha_server_coin,
    pha_server_coin_new,
    pha_server_hosting,
    pha_server_system,
    pha_server_system_new,
    record,
    rights,
    roles,
    server_hosting,
    sms_verification,
    xfx_block_histories,
    xfx_server_system,
    xfx_server_system_new,
);
