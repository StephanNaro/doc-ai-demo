#!/usr/bin/env bash
SECONDS=0
echo -e "These tests require 30 seconds each on my laptop running the llama3.2 model. Your mileage may vary.\n"


echo "  Querying Invoices..."
curl -X POST http://localhost:8001/query \
  -H "Content-Type: application/json" \
  -d '{"query": "What is the total due on INV-2025-001?", "category": "invoices"}'
echo -e "\n"


echo "  Querying Employment Contracts..."
curl -X POST http://localhost:8001/query \
  -H "Content-Type: application/json" \
  -d '{"query": "What is Bob Smiths notice period?", "category": "contracts"}'
echo -e "\n"


echo "  Querying Customer Support..."
curl -X POST http://localhost:8001/query \
  -H "Content-Type: application/json" \
  -d '{"query": "Summarize the damaged product complaint", "category": "support"}'
echo -e "\n"


echo "  Querying Knowledge Base..."
curl -X POST http://localhost:8001/query \
  -H "Content-Type: application/json" \
  -d '{"query": "How many annual leave days for full-time?", "category": "knowledge"}'
echo -e "\n"


echo "  Querying Customer Support..."
curl -X POST http://localhost:8001/query \
  -H "Content-Type: application/json" \
  -d '{"query": "damaged product arrived", "category": "support"}'
echo -e "\n"


duration=$SECONDS
echo "Tests completed. $((duration / 60)) minutes and $((duration % 60)) seconds elapsed."
