#!/bin/bash

echo "🧪 Testing User Role Update Endpoint"
echo "===================================="
echo ""

# Test 1: Update to business_owner
echo "📝 Test 1: Updating role to business_owner"
curl -X PATCH http://127.0.0.1:8000/api/v1/user/role \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "cmgds9uv60000l704ynyfeqs5",
    "role": "business_owner"
  }' | jq '.'

echo ""
echo "===================================="
echo ""

# Test 2: Update to investor
echo "📝 Test 2: Updating role to investor"
curl -X PATCH http://127.0.0.1:8000/api/v1/user/role \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "cmgds9uv60000l704ynyfeqs5",
    "role": "investor"
  }' | jq '.'

echo ""
echo "===================================="
echo ""

# Test 3: Invalid role (should fail)
echo "📝 Test 3: Trying invalid role (should fail)"
curl -X PATCH http://127.0.0.1:8000/api/v1/user/role \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "cmgds9uv60000l704ynyfeqs5",
    "role": "invalid_role"
  }'

echo ""
echo ""
echo "✅ Tests completed!"
