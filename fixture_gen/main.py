import sys
import csv
from datasets import load_dataset

output = str(sys.argv[1])

def count_with_fb():
  x = 10
  y = 0
  if len(sys.argv) >= 3:
    x = int(sys.argv[2])
  if len(sys.argv) >= 4:
    y = int(sys.argv[3])
  return range(y, x + y, 1)


print(len(sys.argv))
print(sys.argv)

count = count_with_fb()


dataset = load_dataset('IlyaGusev/gazeta', revision="v2.0")

with open(output, 'w', newline='') as csv_file:
  fixture_writer = csv.writer(csv_file)
  fixture_writer.writerow(['id', 'text'])
  i = 1
  for x in count:
    entry = dataset['train'][x]['text']
    fixture_writer.writerow([i, entry])
    i += 1
 
