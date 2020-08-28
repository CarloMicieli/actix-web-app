table! {
    brands (brand_id) {
        brand_id -> Uuid,
        name -> Varchar,
        slug -> Varchar,
        brand_logo_id -> Nullable<Uuid>,
        company_name -> Nullable<Varchar>,
        group_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        website_url -> Nullable<Varchar>,
        kind -> Varchar,
        address_line1 -> Nullable<Varchar>,
        address_line2 -> Nullable<Varchar>,
        address_city -> Nullable<Varchar>,
        address_region -> Nullable<Varchar>,
        address_postal_code -> Nullable<Varchar>,
        address_country -> Nullable<Varchar>,
        created -> Timestamp,
        last_modified -> Nullable<Timestamp>,
        version -> Int4,
    }
}

table! {
    catalog_items (catalog_item_id) {
        catalog_item_id -> Uuid,
        brand_id -> Uuid,
        scale_id -> Uuid,
        item_number -> Varchar,
        slug -> Varchar,
        power_method -> Varchar,
        delivery_date -> Nullable<Varchar>,
        available -> Nullable<Bool>,
        description -> Varchar,
        model_description -> Nullable<Varchar>,
        prototype_description -> Nullable<Varchar>,
        created -> Timestamp,
        last_modified -> Nullable<Timestamp>,
        version -> Int4,
    }
}

table! {
    catalog_items_images (catalog_item_id, image_id) {
        catalog_item_id -> Uuid,
        image_id -> Uuid,
        is_default -> Nullable<Bool>,
    }
}

table! {
    collection_items (collection_item_id) {
        collection_item_id -> Uuid,
        collection_id -> Uuid,
        catalog_item_id -> Uuid,
        condition -> Varchar,
        price -> Numeric,
        currency -> Varchar,
        purchased_at -> Nullable<Uuid>,
        added_date -> Timestamp,
        removed_date -> Nullable<Timestamp>,
        notes -> Nullable<Varchar>,
    }
}

table! {
    collections (collection_id) {
        collection_id -> Uuid,
        owner -> Varchar,
        notes -> Nullable<Varchar>,
        created -> Timestamp,
        last_modified -> Nullable<Timestamp>,
        version -> Int4,
    }
}

table! {
    favourite_shops (shop_id, owner) {
        shop_id -> Uuid,
        owner -> Varchar,
    }
}

table! {
    images (image_id) {
        image_id -> Uuid,
        content_type -> Varchar,
        content -> Bytea,
        is_deleted -> Nullable<Bool>,
        created -> Timestamp,
    }
}

table! {
    railways (railway_id) {
        railway_id -> Uuid,
        name -> Varchar,
        company_name -> Nullable<Varchar>,
        slug -> Varchar,
        railway_logo_id -> Nullable<Uuid>,
        country -> Nullable<Varchar>,
        operating_since -> Nullable<Timestamp>,
        operating_until -> Nullable<Timestamp>,
        active -> Nullable<Bool>,
        gauge_mm -> Nullable<Numeric>,
        gauge_in -> Nullable<Numeric>,
        track_type -> Nullable<Varchar>,
        headquarters -> Nullable<Varchar>,
        total_length_mi -> Nullable<Numeric>,
        total_length_km -> Nullable<Numeric>,
        website_url -> Nullable<Varchar>,
        created -> Timestamp,
        last_modified -> Nullable<Timestamp>,
        version -> Int4,
    }
}

table! {
    rolling_stocks (rolling_stock_id) {
        rolling_stock_id -> Uuid,
        catalog_item_id -> Uuid,
        railway_id -> Uuid,
        category -> Varchar,
        epoch -> Varchar,
        min_radius -> Nullable<Numeric>,
        couplers -> Nullable<Varchar>,
        livery -> Nullable<Varchar>,
        length_mm -> Nullable<Numeric>,
        length_in -> Nullable<Numeric>,
        type_name -> Nullable<Varchar>,
        class_name -> Nullable<Varchar>,
        road_number -> Nullable<Varchar>,
        series -> Nullable<Varchar>,
        depot -> Nullable<Varchar>,
        dcc_interface -> Nullable<Varchar>,
        control -> Nullable<Varchar>,
        passenger_car_type -> Nullable<Varchar>,
        service_level -> Nullable<Varchar>,
    }
}

table! {
    scales (scale_id) {
        scale_id -> Uuid,
        name -> Varchar,
        slug -> Varchar,
        ratio -> Numeric,
        gauge_mm -> Nullable<Numeric>,
        gauge_in -> Nullable<Numeric>,
        track_type -> Varchar,
        description -> Nullable<Varchar>,
        standards -> Nullable<Varchar>,
        weight -> Nullable<Int4>,
        created -> Timestamp,
        last_modified -> Nullable<Timestamp>,
        version -> Int4,
    }
}

table! {
    shops (shop_id) {
        shop_id -> Uuid,
        name -> Varchar,
        slug -> Varchar,
        website_url -> Nullable<Varchar>,
        phone_number -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        address_line1 -> Nullable<Varchar>,
        address_line2 -> Nullable<Varchar>,
        address_city -> Nullable<Varchar>,
        address_region -> Nullable<Varchar>,
        address_postal_code -> Nullable<Varchar>,
        address_country -> Nullable<Varchar>,
        created -> Timestamp,
        last_modified -> Nullable<Timestamp>,
        version -> Int4,
    }
}

table! {
    users (user_id) {
        user_id -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
        gravatar_hash -> Nullable<Varchar>,
    }
}

table! {
    wishlist_items (wishlist_item_id) {
        wishlist_item_id -> Uuid,
        wishlist_id -> Uuid,
        catalog_item_id -> Uuid,
        priority -> Varchar,
        added_date -> Timestamp,
        removed_date -> Nullable<Timestamp>,
        price -> Nullable<Numeric>,
        currency -> Nullable<Varchar>,
        notes -> Nullable<Varchar>,
    }
}

table! {
    wishlists (wishlist_id) {
        wishlist_id -> Uuid,
        owner -> Varchar,
        slug -> Varchar,
        wishlist_name -> Nullable<Varchar>,
        visibility -> Varchar,
        budget -> Nullable<Numeric>,
        currency -> Nullable<Varchar>,
        created -> Timestamp,
        last_modified -> Nullable<Timestamp>,
        version -> Int4,
    }
}

joinable!(brands -> images (brand_logo_id));
joinable!(catalog_items -> brands (brand_id));
joinable!(catalog_items -> scales (scale_id));
joinable!(catalog_items_images -> catalog_items (catalog_item_id));
joinable!(catalog_items_images -> images (image_id));
joinable!(collection_items -> catalog_items (catalog_item_id));
joinable!(collection_items -> collections (collection_id));
joinable!(collection_items -> shops (purchased_at));
joinable!(collections -> users (owner));
joinable!(favourite_shops -> shops (shop_id));
joinable!(favourite_shops -> users (owner));
joinable!(railways -> images (railway_logo_id));
joinable!(rolling_stocks -> catalog_items (catalog_item_id));
joinable!(rolling_stocks -> railways (railway_id));
joinable!(wishlist_items -> catalog_items (catalog_item_id));
joinable!(wishlist_items -> wishlists (wishlist_id));
joinable!(wishlists -> users (owner));

allow_tables_to_appear_in_same_query!(
    brands,
    catalog_items,
    catalog_items_images,
    collection_items,
    collections,
    favourite_shops,
    images,
    railways,
    rolling_stocks,
    scales,
    shops,
    users,
    wishlist_items,
    wishlists,
);
