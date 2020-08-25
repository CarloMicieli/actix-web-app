-- // IMAGES ------------

CREATE TABLE public.images
(
    image_id uuid NOT NULL,
    content_type varchar(25) NOT NULL,
    content bytea NOT NULL,
    is_deleted boolean,
    created timestamp without time zone NOT NULL,
    CONSTRAINT "PK_images" PRIMARY KEY (image_id)
);

-- // USERS ------------

CREATE TABLE public.users (
    user_id varchar(25) NOT NULL,
    username varchar(25) NOT NULL,
    email varchar(100) NOT NULL,
    password varchar(25) NOT NULL,
    login_session varchar(36) NOT NULL DEFAULT '',
    gravatar_hash varchar(32) NULL,
    CONSTRAINT "PK_users" PRIMARY KEY (user_id)
);

CREATE UNIQUE INDEX "Idx_users_username"
    ON users USING btree
    (username ASC NULLS LAST);

-- // BRANDS -----------

CREATE TABLE public.brands
(
    brand_id uuid NOT NULL,
    name varchar(25) NOT NULL,
    slug varchar(25) NOT NULL,
    brand_logo_id uuid,
    company_name varchar(50),
    group_name varchar(50),
    description varchar(1000),
    email varchar(100),
    website_url varchar(100),
    kind varchar(25) NOT NULL,
    address_line1 varchar(255),
    address_line2 varchar(255),
    address_city varchar(50),
    address_region varchar(50),
    address_postal_code varchar(10),
    address_country varchar(2),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_brands" PRIMARY KEY (brand_id),
    CONSTRAINT "FK_brands_images" FOREIGN KEY (brand_logo_id)
        REFERENCES public.images (image_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION    
);

CREATE UNIQUE INDEX "Idx_brands_name"
    ON brands USING btree
    (name ASC NULLS LAST);

CREATE UNIQUE INDEX "Idx_brands_slug"
    ON brands USING btree
    (slug ASC NULLS LAST);

-- // RAILWAYS ---------

CREATE TABLE public.railways
(
    railway_id uuid NOT NULL,
    name varchar(25) NOT NULL,
    company_name varchar(250),
    slug varchar(25) NOT NULL,
    railway_logo_id uuid,
    country varchar(2),
    operating_since timestamp without time zone,
    operating_until timestamp without time zone,
    active boolean,
    gauge_mm numeric(19,5),
    gauge_in numeric(19,5),
    track_type varchar(25),
    headquarters varchar(250),
    total_length_mi numeric(19,5),
    total_length_km numeric(19,5),
    website_url varchar(255),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_railways" PRIMARY KEY (railway_id),
    CONSTRAINT "FK_railways_images" FOREIGN KEY (railway_logo_id)
    REFERENCES public.images (image_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE UNIQUE INDEX "Idx_railways_name"
    ON public.railways USING btree
    (name ASC NULLS LAST);

CREATE UNIQUE INDEX "Idx_railways_slug"
    ON public.railways USING btree
    (slug ASC NULLS LAST);    

-- // SCALES -----------

CREATE TABLE public.scales
(
    scale_id uuid NOT NULL,
    name varchar(25) NOT NULL,
    slug varchar(25) NOT NULL,
    ratio numeric(19,5) NOT NULL,
    gauge_mm numeric(19,5),
    gauge_in numeric(19,5),
    track_type varchar(25) NOT NULL,
    description varchar(250),
    standards varchar(100),
    weight integer,
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_scales" PRIMARY KEY (scale_id)
);

CREATE UNIQUE INDEX "Idx_scales_name"
    ON public.scales USING btree
    (name ASC NULLS LAST);

CREATE UNIQUE INDEX "Idx_scales_slug"
    ON public.scales USING btree
    (slug ASC NULLS LAST);

-- // CATALOG ITEMS -----------

CREATE TABLE public.catalog_items
(
    catalog_item_id uuid NOT NULL,
    brand_id uuid NOT NULL,
    scale_id uuid NOT NULL,
    item_number varchar(10) NOT NULL,
    slug varchar(40) NOT NULL,
    power_method varchar(2) NOT NULL,
    delivery_date varchar(10),
    available boolean,
    description varchar(250) NOT NULL,
    model_description varchar(2500),
    prototype_description varchar(2500),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_catalog_items" PRIMARY KEY (catalog_item_id),
    CONSTRAINT "FK_catalog_items_brands" FOREIGN KEY (brand_id)
        REFERENCES public.brands (brand_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_catalog_items_scales" FOREIGN KEY (scale_id)
        REFERENCES public.scales (scale_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE UNIQUE INDEX "Idx_catalog_items_brand_id_item_number"
    ON public.catalog_items USING btree
    (item_number ASC NULLS LAST, brand_id ASC NULLS LAST);

CREATE UNIQUE INDEX "Idx_catalog_items_slug"
    ON public.catalog_items USING btree
    (slug ASC NULLS LAST);


-- // CATALOG IMAGES -----------

CREATE TABLE public.catalog_items_images
(
    catalog_item_id uuid NOT NULL,
    image_id uuid NOT NULL,
    is_default boolean,
    CONSTRAINT "PK_catalog_items_images" PRIMARY KEY (catalog_item_id, image_id),
    CONSTRAINT "FK_catalog_items_images_catalog_items" FOREIGN KEY (catalog_item_id)
        REFERENCES public.catalog_items (catalog_item_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_catalog_items_images_images" FOREIGN KEY (image_id)
        REFERENCES public.images (image_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

-- // ROLLING STOCKS -----------

CREATE TABLE public.rolling_stocks
(
    rolling_stock_id uuid NOT NULL,
    catalog_item_id uuid NOT NULL,
    railway_id uuid NOT NULL,
    category varchar(25) NOT NULL,
    epoch varchar(10) NOT NULL,
    min_radius numeric(19,5),
    couplers varchar(10),
    livery varchar(50),
    length_mm numeric(19,5),
    length_in numeric(19,5),
    type_name varchar(25),
    class_name varchar(15),
    road_number varchar(15),
    series varchar(50),
    depot varchar(100),
    dcc_interface varchar(10),
    control varchar(10),
    passenger_car_type varchar(25),
    service_level varchar(15),
    CONSTRAINT "PK_rolling_stocks" PRIMARY KEY (rolling_stock_id),
    CONSTRAINT "FK_rolling_stocks_catalog_items" FOREIGN KEY (catalog_item_id)
        REFERENCES public.catalog_items (catalog_item_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_rolling_stocks_railways" FOREIGN KEY (railway_id)
        REFERENCES public.railways (railway_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

-- // SHOPS -----------

CREATE TABLE public.shops
(
    shop_id uuid NOT NULL,
    name varchar(50) NOT NULL,
    slug varchar(50) NOT NULL,
    website_url varchar(100),
    phone_number varchar(50),
    email varchar(100),
    address_line1 varchar(255),
    address_line2 varchar(255),
    address_city varchar(50),
    address_region varchar(50),
    address_postal_code varchar(10),
    address_country varchar(2),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_shops" PRIMARY KEY (shop_id)
);

CREATE INDEX "Idx_shops_slug"
    ON public.shops USING btree
    (slug ASC NULLS LAST);

-- // favourite shops -----------

CREATE TABLE public.favourite_shops
(
    shop_id uuid NOT NULL,
    owner varchar(25) NOT NULL,
    CONSTRAINT "PK_favourite_shops" PRIMARY KEY (shop_id, owner),
    CONSTRAINT "FK_favourite_shops_users" FOREIGN KEY (owner)
        REFERENCES public.users (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_favourite_shops_shop_id" FOREIGN KEY (shop_id)
        REFERENCES public.shops (shop_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

-- // collections -----------

CREATE TABLE public.collections
(
    collection_id uuid NOT NULL,
    owner varchar(25) NOT NULL,
    notes varchar(250),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_collections" PRIMARY KEY (collection_id),
    CONSTRAINT "FK_collections_users" FOREIGN KEY (owner)
        REFERENCES public.users (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE INDEX "Idx_collections_owner"
    ON public.collections USING btree
    (owner ASC NULLS LAST);

-- // collection items -----------

CREATE TABLE public.collection_items
(
    collection_item_id uuid NOT NULL,
    collection_id uuid NOT NULL,
    catalog_item_id uuid NOT NULL,
    condition varchar(15) NOT NULL,
    price numeric(19,5) NOT NULL,
    currency varchar(3) NOT NULL,
    purchased_at uuid,
    added_date timestamp without time zone NOT NULL,
    removed_date timestamp without time zone,
    notes varchar(150),
    CONSTRAINT "PK_collection_items" PRIMARY KEY (collection_item_id),
    CONSTRAINT "FK_collection_items_catalog_items" FOREIGN KEY (catalog_item_id)
        REFERENCES public.catalog_items (catalog_item_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_collection_items_collections" FOREIGN KEY (collection_id)
        REFERENCES public.collections (collection_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_collection_items_shops" FOREIGN KEY (purchased_at)
        REFERENCES public.shops (shop_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

-- // wishlists -----------

CREATE TABLE public.wishlists
(
    wishlist_id uuid NOT NULL,
    owner varchar(25) NOT NULL,
    slug varchar(100) NOT NULL,
    wishlist_name varchar(100),
    visibility varchar(15) NOT NULL,
    budget numeric(19,5),
    currency varchar(3),
    created timestamp without time zone NOT NULL,
    last_modified timestamp without time zone,
    version integer NOT NULL DEFAULT 1,
    CONSTRAINT "PK_wishlists" PRIMARY KEY (wishlist_id),
    CONSTRAINT "FK_wishlists_users" FOREIGN KEY (owner)
        REFERENCES public.users (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE INDEX "Idx_wishlists_slug"
    ON public.wishlists USING btree
    (slug ASC NULLS LAST);

-- // wishlist items -----------    

CREATE TABLE public.wishlist_items
(
    wishlist_item_id uuid NOT NULL,
    wishlist_id uuid NOT NULL,
    catalog_item_id uuid NOT NULL,
    priority varchar(10) NOT NULL,
    added_date timestamp without time zone NOT NULL,
    removed_date timestamp without time zone,
    price numeric(19,5),
    currency varchar(3),
    notes varchar(150),
    CONSTRAINT "PK_wishlist_items" PRIMARY KEY (wishlist_item_id),
    CONSTRAINT "FK_wishlist_items_catalog_items" FOREIGN KEY (catalog_item_id)
        REFERENCES public.catalog_items (catalog_item_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_wishlist_items_wishlists" FOREIGN KEY (wishlist_id)
        REFERENCES public.wishlists (wishlist_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);