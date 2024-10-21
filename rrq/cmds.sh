###############################################################
# Health
###############################################################

curl -X GET https://rrq.lumina.sh

###############################################################
# Information
###############################################################

curl -X GET http://0.0.0.0:8000/information


###############################################################
# Produce
###############################################################

curl -X POST http://0.0.0.0:8000/produce \
-H "Content-Type: application/json" \
-d '[
  {
    "max_attempts": 3,
    "queue_name": "example1",
    "publish_channel": "channel1",
    "payload": {"key": "value"},
    "item_id": "b5b759cc-bc40-4da9-af5d-c0530954a648"
  },
  {
    "max_attempts": 4,
    "queue_name": "example1",
    "publish_channel": "channel3",
    "payload": {"key4": "value4"},
    "item_id": "item-id-1"
  }
]'

###############################################################
# Consume
###############################################################

curl -X POST http://0.0.0.0:8000/consume \
-H "Content-Type: application/json" \
-d '{
    "consumer_id": "random",
    "queue_name": "example1",
    "item_count": 3,
    "expiration_seconds": 6000
}'

curl -X POST http://0.0.0.0:8000/consume \
-H "Content-Type: application/json" \
-d '{
    "consumer_id": "1",
    "queue_name": "file_ids_test",
    "item_count": 3,
    "expiration_seconds": 60
}'

###############################################################
# Compelete
###############################################################
curl -X POST http://0.0.0.0:8000/complete \
-H "Content-Type: application/json" \
-d '[
  {
    "item_id": "04067826c-d1b2-496e-a5e3-e810d0c5d0b2",
    "consumer_id": "random",
    "queue_name": "example1",
    "item_index": 1,
    "result": "Success",
    "message": null
  }
]'
