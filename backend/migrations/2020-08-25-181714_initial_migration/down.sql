DROP TABLE public.wishlist_items;

DROP INDEX public."Idx_wishlists_slug";
DROP TABLE public.wishlists;

DROP TABLE public.collection_items;

DROP INDEX public."Idx_collections_owner";
DROP TABLE public.collections;

DROP TABLE public.favourite_shops;

DROP INDEX public."Idx_shops_slug";
DROP TABLE public.shops;

DROP TABLE public.rolling_stocks;
DROP TABLE public.catalog_items_images;

DROP INDEX public."Idx_catalog_items_slug";
DROP INDEX public."Idx_catalog_items_brand_id_item_number";
DROP TABLE public.catalog_items;

DROP INDEX public."Idx_scales_slug";
DROP INDEX public."Idx_scales_name";
DROP TABLE public.scales;

DROP INDEX public."Idx_railways_name";
DROP INDEX public."Idx_railways_slug";
DROP TABLE public.railways;

DROP INDEX public."Idx_brands_name";
DROP INDEX public."Idx_brands_slug";
DROP TABLE public.brands;

DROP INDEX public."Idx_users_username";
DROP TABLE public.users;

DROP TABLE public.images;