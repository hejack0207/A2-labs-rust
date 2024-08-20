#!/usr/bin/env -S bats --show-output-of-passing-tests

base_url="http://${WEB_HOST:-127.0.0.1}:3000"

query_with_content_type() {
	curl -q \
	    -H "Content-Type: $1" \
	    -X POST \
	    --raw \
	    -d "$2" \
	    -s -S \
	    --insecure \
	    -m 5 \
	    ${base_url}"$3"
}

query() {
	query_with_content_type "application/json" "$1" "$2"
}

grepstr() {
	declare result=$1
	shift
	printf "%s" "$result" | grep "$@" >/dev/null
}

printrep() {
	declare query=$1
	declare result=$2
	printf "Sent:\n"
	printf "%s\n" "$query"
	printf "Received:\n"
	printf "%s\n" "$result"
}

# "opcOrOp": [1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
# "opcOrOp": "$rand32",
@test "on model init" {
	rand20=$(openssl rand -hex 10)
	rand32=$(openssl rand -hex 16)
	curtime=$(date +'%Y-%m-%d %H:%M:%S')
	query="$(cat <<EOI | tr -d '\n'
{
	"header": {
		"simulationId": "$rand20",
		"sceneObjectId": "$curtime",
		"simulationTime": 1111,
		"timestamp": 2222
	},
	"body": {
        	"ueid": 1,
        	"sst": 1,
        	"sd": 1,
		"cyclicPrefix": 0,
		"subCarrierSpacing": 3,
		"routeAddIp": "192.168.0.1",
		"usimMode": 0,
		"authAlgo": 1,
		"opType": 1,
		"opcOrOp": "$rand32",
		"k": "$rand32",
		"imsi": "086123456789012",
		"imei": "086123456789012",
		"msisdn": "12345678901",
		"imeisv": "1234567890123456",
		"dnn": "cscnnet",
		"latitude": -60,
		"longitude": 45,
		"altitude": 30
	}
}
EOI
)"
	result=$(query "$query" "/model/init")
	printrep "$query" "$result"
	grepstr "$result" "200"
}

@test "on model config" {
	# skip
	rand20=$(openssl rand -hex 10)
	rand32=$(openssl rand -hex 16)
	curtime=$(date +'%Y-%m-%d %H:%M:%S')
	query="$(cat <<EOI | tr -d '\n'
{
	"header": {
		"simulationId": "$rand20",
		"sceneObjectId": "$curtime",
		"simulationTime": 1111,
		"timestamp": 2222
	},
	"body": {
		"optType": 1,
		"capacity": 2,
		"serviceAddr": "192.168.0.1",
		"phoneNum": "861234567890"
	}
}
EOI
)"
	result=$(query "$query" "/model/config")
	printrep "$query" "$result"
	grepstr "$result" "200"
}
