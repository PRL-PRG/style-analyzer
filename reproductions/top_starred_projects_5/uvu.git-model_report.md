# Model report for file:///tmp/top-repos-quality-repos-y4ywcjea/uvu.git HEAD d56753ec20928da1c22ff093cb07ce3c67613706

### Dump

```json
{'created_at': '2021-08-29 22:40:34',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.6 kB',
 'tags': [],
 'uuid': '5ef3ef31-2270-48f7-be44-39300b0d9e22',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-y4ywcjea/uvu.git d56753ec20928da1c22ff093cb07ce3c67613706

# javascript
28 rules, avg.len. 7.4
## train
PPCR: 0.949863
### report
macro
{'f1-score': 0.9204932705007485,
 'precision': 0.9410636300283228,
 'recall': 0.9024424742456196,
 'support': 23871}
micro
{'f1-score': 0.9615432952117632,
 'precision': 0.9615432952117632,
 'recall': 0.9615432952117632,
 'support': 23871}
weighted
{'f1-score': 0.9612359952872762,
 'precision': 0.9617589299736133,
 'recall': 0.9615432952117632,
 'support': 23871}
### report_full
macro
{'f1-score': 0.8799785134233875,
 'precision': 0.9410636300283228,
 'recall': 0.8321932261841035,
 'support': 25131}
micro
{'f1-score': 0.9368189053508019,
 'precision': 0.9615432952117632,
 'recall': 0.9133341291631849,
 'support': 25131}
weighted
{'f1-score': 0.9353899421492212,
 'precision': 0.9607565370392003,
 'recall': 0.9133341291631849,
 'support': 25131}
## test
PPCR: 0.969540
### report
macro
{'f1-score': 0.9187537896043096,
 'precision': 0.9371941030347183,
 'recall': 0.903711676268367,
 'support': 5443}
micro
{'f1-score': 0.9676648906852838,
 'precision': 0.9676648906852838,
 'recall': 0.9676648906852838,
 'support': 5443}
weighted
{'f1-score': 0.9672848445125759,
 'precision': 0.9679306255739312,
 'recall': 0.9676648906852838,
 'support': 5443}
### report_full
macro
{'f1-score': 0.8965309476152773,
 'precision': 0.9371941030347183,
 'recall': 0.8648908385039221,
 'support': 5614}
micro
{'f1-score': 0.9526996472822646,
 'precision': 0.9676648906852838,
 'recall': 0.9381902386889918,
 'support': 5614}
weighted
{'f1-score': 0.9513296000955701,
 'precision': 0.9668091725324728,
 'recall': 0.9381902386889918,
 'support': 5614}
```

## javascript
### Summary
17 rules, avg.len. 7.1

| | |
|-|-|
|Min support|122|
|Max support|3523|
|Min confidence|0.9277523159980774|
|Max confidence|0.9996304512023926|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 1.000. Support: 1353.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {CALL}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 377.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ -4.roles in {BINARY}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ⏎<br>Confidence: 0.994. Support: 269.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.994. Support: 1327.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.996. Support: 336.` |
| 6 | `  •••start_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎⏎<br>Confidence: 0.964. Support: 266.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 995.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 603.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 548.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.928. Support: 436.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 526.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {MAP}<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {CALLEE}<br>	∧ ^1.roles in {EXPRESSION} and not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 3487.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION, MAP}<br>	∧ -1.length ≤ 1<br>	∧ -2.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {CALLEE}<br>	∧ ^1.roles in {EXPRESSION} and not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 3523.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 152.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 122.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 143.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles not in {KEY, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 2375.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.117647058823529, "max_conf": 0.9996304512023926, "max_support": 3523, "min_conf": 0.9277523159980774, "min_support": 122, "num_rules": 17}}
```
</details>
