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

### Accounts

* `POST /api/v1/accounts`: create a new account

```json
{
  "username": "string",
  "password": "string",
  "email": "string",
  "favourites": 
  {
    "brand": "string",
    "railway": "string",
    "scale": "string"
  }
}
```

* `POST /api/v1/authenticate`: authenticate the user and returns back a JWT token for following requests

```json
{
  "username": "string",
  "password": "string"
}
```

### Brands

A model railways producer.

* `GET /api/v1/brands?start=0&limit=25`: returns the list of brands - sorted by their name.

```json
{
  "_links": {
    "_self": "string",
    "prev": "string",
    "next": "string"
  },
  "limit": 0,
  "results": [
    {
      "id": "string",
      "name": "string",
      "company_name": "string",
      "logo_url": "string",
      "address":
      {
        "line1": "string",
        "line2": "string",
        "city": "string",
        "region": "string",
        "country": "string",
        "postal_code": "string"
      },
      "mail_address": "string",
      "website_url": "string",
      "kind": "string"
    }
  ]
}
```

* `GET /api/v1/brands/{brand}`: return the information for the brand with `{brand}` slug.

```json
{
    "id": "string",
    "name": "string",
    "company_name": "string",
    "logo_url": "string",
    "address":
    {
      "line1": "string",
      "line2": "string",
      "city": "string",
      "region": "string",
      "country": "string",
      "postal_code": "string"
    },
    "mail_address": "string",
    "website_url": "string",
    "kind": "string"
}
```

* `POST /api/v1/brands`: create a new brand (only authenticated user).

```json
{
    "name": "string",
    "company_name": "string",
    "address":
    {
      "line1": "string",
      "line2": "string",
      "city": "string",
      "region": "string",
      "country": "string",
      "postal_code": "string"
    },
    "mail_address": "string",
    "website_url": "string",
    "kind": "string"
}
```

### Railways

* `GET /api/v1/railways`: returns the list of railways - sorted by their names.

```json
{
  "_links": {
    "_self": "string",
    "prev": "string",
    "next": "string"
  },
  "limit": 0,
  "results": [
    {
      "_links": 
      {
        "_self": "string",
        "slug": "string"
      },
      "id": "string",
      "name": "string",
      "company_name": "string",
      "description": "string",
      "logo_url": "string",
      "country": "string",
      "period_of_activity":
      {
        "status": "string",
        "operating_since": "datetime",
        "operating_until": "datetime"
      },
      "total_length": 
      {
        "miles": "number",
        "kilometers": "number"
      },
      "gauge": 
      {
        "millimeters": "number",
        "inches": "number",
        "track_gauge": "string"
      },
      "headquarters": "string",
      "website_url": "string"
    }
  ]
}
```

* `GET /api/v1/railways/{railway}`: return the information for the railway with `{railway}` slug.
* `POST /api/v1/railway`: create a new railway (only authenticated user).

### Scales

* `GET /api/v1/scales`: returns the list of scales - sorted by their name.

```json
{
  "_links": {
    "_self": "string",
    "prev": "string",
    "next": "string"
  },
  "limit": "number",
  "results": [
    {
      "_links": {
        "_self": "string",
        "slug": "string"
      },
      "id": "string",
      "name": "string",
      "ratio": "number",
      "gauge": {
        "millimeters": "number",
        "inches": "number",
        "track_gauge": "string"
      },
      "description": "string",
      "standards": [
        "string"
      ]
    }
  ]
}
```

* `GET /api/v1/scales/{scale}`: return the information for the scale with `{scale}` slug.
* `POST /api/v1/scale`: create a new scale (only authenticated user).

### Catalog items

* `GET /api/v1/catalog-items`: returns the list of catalog items - sorted by their name.

```json
{
  "_links": {
    "_self": "string",
    "prev": "string",
    "next": "string"
  },
  "limit": 0,
  "results": [
    {
      "_links": {
        "_self": "string",
        "slug": "string"
      },
      "id": "string",
      "item_number": "string",
      "brand": {
        "id": "string",
        "name": "string",
        "slug": "string"
      },
      "description": "string",
      "prototype_description": "string",
      "model_description": "string",
      "image_url": "string",
      "delivery_date": "string",
      "available": true,
      "scale": {
        "id": "string",
        "name": "string",
        "slug": "string"
      },
      "power_method": "string",
      "rolling_stocks": [
        {
          "id": "string",
          "railway": {
            "id": "string",
            "name": "string",
            "slug": "string"
          },
          "category": "string",
          "epoch": "string",
          "length_over_buffer": {
            "millimeters": "number",
            "inches": "number"
          },
          "min_radius": {
            "millimeters": "number"
          },
          "class_name": "string",
          "road_number": "string",
          "series": "string",
          "type_name": "string",
          "couplers": "string",
          "livery": "string",
          "depot": "string",
          "passenger_car_type": "string",
          "service_level": "string",
          "dcc_interface": "string",
          "control": "string"
        }
      ]
    }
  ]
}
```

* `GET /api/v1/catalog-items/{catalogItem}`: return the information for the catalog item with `{catalogItem}` slug.
* `POST /api/v1/catalog-items`: create a new catalog item (only authenticated user).

### Collections

* `GET /api/v1/collections/{id}`: returns the collection with id `{id}` - only when the current user is the owner.

```json
{
  "id": "string",
  "owner": "string",
  "items": [
    {
      "item_id": "string",
      "catalog_item": {
        "catalog_item_id": "string",
        "slug": "string",
        "brand": "string",
        "item_number": "string",
        "category": "string",
        "description": "string"
      },
      "condition": "string",
      "price": {
        "amount": "number",
        "currency": "string"
      },
      "purchased_at": {
        "shop_id": "string",
        "slug": "string",
        "name": "string"
      },
      "added_date": "date",
      "removed_date": "date",
      "notes": "string"
    }
  ],
  "size": "number"
}
```

* `POST /api/v1/collections/{id}/items`: add a new item to the collection with id `{id}`

```json
{
  "catalog_item": "string",
  "shop": "string",
  "price": {
    "amount": "number",
    "currency": "string"
  },
  "condition": "string",
  "added_date": "date",
  "notes": "string"
}
```

* `PUT /api/v1/collections/{id}/items/{itemId}`: modify the item `{itemId}` in the collection with id `{id}`

```json
{
  "shop": "string",
  "price": {
    "amount": "number",
    "currency": "string"
  },
  "condition": "string",
  "added_date": "date",
  "removed_date": "date",
  "notes": "string"
}
```

* `DELETE /api/v1/collections/{id}/items/{itemId}`: removes the item `{itemId}` from the collection `{id}`

* `GET /api/v1/collections/{id}/statistics`: returns the statistics for the collection with id `{id}` - only when the current user is the owner.

```json
{
  "id": "string",
  "owner": "string",
  "modified_date": "date",
  "total_value": {
    "amount": "number",
    "currency": "string"
  },
  "details": [
    {
      "category": "string",
      "count": "number",
      "year": "number",
      "total_value": {
        "amount": "number",
        "currency": "string"
      }
    }
  ]
}
```

### Wishlists

* `GET /api/v1/wishlists/owner/{owner}?visibility=:visibility`: returns the wishlist for the `{owner}` user with the given visibility. Only the owner is allowed to see the private wishlists.

```json
{
  "owner": "string",
  "visibility": "string",
  "lists": [
    {
      "id": "string",
      "slug": "string",
      "list_name": "string",
      "visibility": "string"
    }
  ]
}
```

* `GET /api/v1/wishlists/{id}`: returns the wishlist with the `{id}` id.

```json
{
  "wishlist": {
    "id": "string",
    "slug": "string",
    "list_name": "string",
    "visibility": "string",
    "budget": {
      "value": "number",
      "currency": "string"
    },
    "owner": "string",
    "items": [
      {
        "id": "string",
        "priority": "string",
        "added_date": "date",
        "removed_date": "date",
        "catalog_item": {
          "id": "string",
          "slug": "string",
          "brand": "string",
          "item_number": "string",
          "count": "number",
          "category": "number",
          "description": "string"
        },
        "notes": "string"
      }
    ],
    "size": "number"
  }
}
```

* `POST /api/v1/wishlists`: creates a new wishlist

```json
{
  "list_name": "string",
  "visiblity": "string",
  "budget": {
    "value": "number",
    "currency": "string"
  }
}
```

* `PUT /api/v1/wishlists/{id}`: modify the wishlist with `{id}`

```json
{
  "list_name": "string",
  "visibility": "string"
}
```

* `DELETE /api/v1/wishlists/{id}`: removes the wishlist with `{id}`

* `POST /api/v1/wishlists/{id}/items`: add a new item to the wishlist with `{id}`.

```json
{
  "catalog_item": "string",
  "priority": "string",
  "notes": "string"
}
```

* `PUT /api/v1/wishlists/{id}/items/{itemId}`: modify the wishlist item `{itemId}` in this wishlist

```json
{
  "priority": "string",
  "notes": "string"
}
```

* `DELETE /api/v1/wishlists/{id}/items/{itemId}`: removes the wishlist item `{itemId}` from this wishlist
