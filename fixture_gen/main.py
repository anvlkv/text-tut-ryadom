import sys
import csv
from transformers import pipeline

def count_with_fb():
  x = 10
  if len(sys.argv) > 3:
    x = int(sys.argv[2])
  return range(0, x, 1)

def prompt_with_fb():
  x = "Расплывчатый текст о игуанах"
  if len(sys.argv) > 4:
    x = sys.argv[3]
  return x

output = str(sys.argv[1])
count = count_with_fb()
prompt = prompt_with_fb()

pipe = pipeline("text-generation", model="mistralai/Mixtral-8x7B-Instruct-v0.1")

with open(output, 'w', newline='') as csv_file:
  fixture_writer = csv.writer(csv_file)
  fixture_writer.writerow(['id', 'text'])
  i = 1
  for x in count:
    entry = pipe(prompt)
    fixture_writer.writerow([i, entry])
    i += 1
 
