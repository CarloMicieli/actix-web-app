# Trenako

## Frontend

* `/`: homepage - welcome page and project highlights, latest 10 rolling stocks added
* `/you`: user profile page - only for authenticated user - see something from collection and wishlists. Sidebar with collection categories and wishlists. Display a button to create new wishlists. Users are given a collection automatically when they sign up.
* `/you/collection`: display the collection - on the top display collection overview and statistics. On the bottom display the items inside this collection (with pagination).
* `/you/wishlists/:wishlist`: display the wishlist with `:wishlist` slug - on the top display wishlist overview and statistics. On the bottom display the items inside this wishlist (with pagination). Users can modify priority.
* `/browse/brands`: display the brands list. Authenticated users will see their favourite brands on top. Users can add/remove brands to their favourites.
* `/browse/brands/:brand`: display the detailed information for the brand with the `:brand` slug. Users can add/remove this brand to their favourites.
* `/browse/railways`: display the railways list. Authenticated users will see their favourite railways on top. Users can add/remove railways to their favourites. 
* `/browse/railways/:railway`: display the detailed information for the railway with the `:railway` slug. Users can add/remove this railway to their favourites.
* `/browse/scales`: display the scales list. Authenticated users will see their favourite scales on top. Users can add/remove scales to their favourites. 
* `/browse/scales/:scale`: display the detailed information for the scale with the `:scale` slug. Users can add/remove this scale to their favourites.
* `/browse/catalog`: users will browse catalog items managing various filters - by brand, category, power method, scale.
* `/browse/catalog/:catalog`: display the detailed information for the catalog item with the `:catalog` slug. Users can add/remove this item to their collections or wishlists.
* `/stats`: application statistics
* `/new-rolling-stock`: add a new rolling stock to the database

## Backend

### Brands

A model railways producer.

* `GET /api/v1/brands`: returns the list of brands - sorted by their name.
* `GET /api/v1/brands/:brand`: return the information for the brand with `:brand` slug.
* `POST /api/v1/brands`: create a new brand (only authenticated user).

### Railways

* `GET /api/v1/railways`: returns the list of railways - sorted by their name.
* `GET /api/v1/railways/:railway`: return the information for the railway with `:railway` slug.
* `POST /api/v1/railway`: create a new railway (only authenticated user).

### Scales

* `GET /api/v1/scales`: returns the list of scales - sorted by their name.
* `GET /api/v1/scales/:scale`: return the information for the scale with `:scale` slug.
* `POST /api/v1/scale`: create a new scale (only authenticated user).

### Catalog items

* `GET /api/v1/catalog-items?`: returns the list of catalog items - sorted by their name.
* `GET /api/v1/catalog-items/:catalog-item`: return the information for the catalog item with `:catalog-item` slug.
* `POST /api/v1/catalog-items`: create a new catalog item (only authenticated user).

### Collections

### Wishlists