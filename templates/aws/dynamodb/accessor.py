import boto3
from botocore.exceptions import ClientError

# DynamoDBクライアントの作成
dynamodb = boto3.client('dynamodb')

# 1. 単一項目の取得 (GetItem)
def get_item(table_name, key):
    try:
        response = dynamodb.get_item(
            TableName=table_name,
            Key=key
        )
        return response.get('Item')
    except ClientError as e:
        print(e.response['Error']['Message'])

# 2. 項目の追加または更新 (PutItem)
def put_item(table_name, item):
    try:
        dynamodb.put_item(
            TableName=table_name,
            Item=item
        )
    except ClientError as e:
        print(e.response['Error']['Message'])

# 3. 項目の削除 (DeleteItem)
def delete_item(table_name, key):
    try:
        dynamodb.delete_item(
            TableName=table_name,
            Key=key
        )
    except ClientError as e:
        print(e.response['Error']['Message'])

# 4. クエリ (Query)
def query_items(table_name, key_condition_expression, expression_attribute_values):
    try:
        response = dynamodb.query(
            TableName=table_name,
            KeyConditionExpression=key_condition_expression,
            ExpressionAttributeValues=expression_attribute_values
        )
        return response.get('Items')
    except ClientError as e:
        print(e.response['Error']['Message'])

# 5. スキャン (Scan)
def scan_table(table_name, filter_expression=None, expression_attribute_values=None):
    try:
        if filter_expression:
            response = dynamodb.scan(
                TableName=table_name,
                FilterExpression=filter_expression,
                ExpressionAttributeValues=expression_attribute_values
            )
        else:
            response = dynamodb.scan(TableName=table_name)
        return response.get('Items')
    except ClientError as e:
        print(e.response['Error']['Message'])

# 6. トランザクション (TransactWriteItems)
def transact_write_items(transact_items):
    try:
        dynamodb.transact_write_items(
            TransactItems=transact_items
        )
    except ClientError as e:
        print(e.response['Error']['Message'])

# 7. 条件付き書き込み (Conditional Writes)
def conditional_put_item(table_name, item, condition_expression, expression_attribute_values):
    try:
        dynamodb.put_item(
            TableName=table_name,
            Item=item,
            ConditionExpression=condition_expression,
            ExpressionAttributeValues=expression_attribute_values
        )
    except ClientError as e:
        print(e.response['Error']['Message'])
