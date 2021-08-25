# Model report for file:///tmp/top-repos-quality-repos-0pxfcpy1/crucible.git HEAD d503264b38931d00b234fe26882541d8c728b2e1

### Dump

```json
{'created_at': '2021-08-20 18:35:00',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '30.7 kB',
 'tags': [],
 'uuid': 'd9189354-b1f9-4f40-98d5-8ab9a7d682ef',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-0pxfcpy1/crucible.git d503264b38931d00b234fe26882541d8c728b2e1

# javascript
45 rules, avg.len. 8.3
## train
PPCR: 0.935498
### report
macro
{'f1-score': 0.3370671084113624,
 'precision': 0.35470647957643076,
 'recall': 0.32527224325056936,
 'support': 98304}
micro
{'f1-score': 0.9603474934895834,
 'precision': 0.9603474934895834,
 'recall': 0.9603474934895834,
 'support': 98304}
weighted
{'f1-score': 0.9564559020302289,
 'precision': 0.9541152849171809,
 'recall': 0.9603474934895834,
 'support': 98304}
### report_full
macro
{'f1-score': 0.31056338174623793,
 'precision': 0.35470647957643076,
 'recall': 0.28883712577122117,
 'support': 105082}
micro
{'f1-score': 0.928343150462667,
 'precision': 0.9603474934895834,
 'recall': 0.8984031518242896,
 'support': 105082}
weighted
{'f1-score': 0.9164521874630827,
 'precision': 0.9416792458921932,
 'recall': 0.8984031518242896,
 'support': 105082}
## test
PPCR: 0.929201
### report
macro
{'f1-score': 0.33615803684580486,
 'precision': 0.3499314482486133,
 'recall': 0.3258143135079259,
 'support': 18348}
micro
{'f1-score': 0.9577610638761718,
 'precision': 0.9577610638761718,
 'recall': 0.9577610638761718,
 'support': 18348}
weighted
{'f1-score': 0.9530371824005065,
 'precision': 0.9496450274356355,
 'recall': 0.9577610638761718,
 'support': 18348}
### report_full
macro
{'f1-score': 0.30910004583299006,
 'precision': 0.3499314482486133,
 'recall': 0.2879607898409418,
 'support': 19746}
micro
{'f1-score': 0.9226124849057594,
 'precision': 0.9577610638761718,
 'recall': 0.8899523954218576,
 'support': 19746}
weighted
{'f1-score': 0.9077763435739695,
 'precision': 0.9326576902743929,
 'recall': 0.8899523954218576,
 'support': 19746}
```

## javascript
### Summary
32 rules, avg.len. 8.1

| | |
|-|-|
|Min support|97|
|Max support|16970|
|Min confidence|0.9203612208366394|
|Max confidence|0.9990825653076172|

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
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = module<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.reserved = ;<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 545.` |
| 2 | `  -1.reserved not in {), module}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.reserved = ;<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 141.` |
| 3 | `  -1.reserved not in {)}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.reserved not in {;}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 16970.` |
| 4 | `  -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 1030.` |
| 5 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 4180.` |
| 6 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.983. Support: 2979.` |
| 7 | `  -1.diff_offset ≥ 2<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 643.` |
| 8 | `  •••start_line ≥ 100<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_offset ≥ 9<br>	∧ -4.label in {<newline>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 264.` |
| 9 | `  •••start_line ≥ 100<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.reserved not in {(, {}<br>	∧ -4.label not in {<newline>}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 2963.` |
| 10 | `  •••start_col ≥ 12<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = '<br>Confidence: 0.996. Support: 114.` |
| 11 | `  •••start_col ≤ 12<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +4.length ≥ 4<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.960. Support: 187.` |
| 12 | `  •••start_col ≤ 12<br>	∧ •••start_line ≤ 253<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +4.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.935. Support: 100.` |
| 13 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = '<br>Confidence: 0.985. Support: 820.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 10<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 438.` |
| 15 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {STRING}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, STATEMENT}<br>⇒ y = '<br>Confidence: 0.985. Support: 97.` |
| 16 | `  •••start_col ≥ 6<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {FILE, IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 203.` |
| 17 | `  •••start_col ≥ 6<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.roles in {STRING}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {FILE, IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 399.` |
| 18 | `  •••start_col ≥ 6<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.roles not in {STRING}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 7833.` |
| 19 | `  -1.roles not in {LITERAL}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.980. Support: 3071.` |
| 20 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 3099.` |
| 21 | `  -1.roles in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.990. Support: 2107.` |
| 22 | `  -1.label in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 155.` |
| 23 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 609.` |
| 24 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≤ 7<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 124.` |
| 25 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = ?<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.974. Support: 251.` |
| 26 | `  -1.reserved = return<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 135.` |
| 27 | `  -1.reserved not in {return}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.936. Support: 272.` |
| 28 | `  -1.reserved not in {return}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {:, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 378.` |
| 29 | `  -1.reserved not in {return}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved = ,<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.948. Support: 106.` |
| 30 | `  -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {), ;, ?, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 470.` |
| 31 | `  -1.reserved not in {), return}<br>	∧ -1.roles not in {STRING}<br>	∧ -5.length ≥ 5<br>	∧ +1.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 936.` |
| 32 | `  -1.reserved not in {), return}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {:, ;, ?, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {VALUE}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 16035.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.125, "max_conf": 0.9990825653076172, "max_support": 16970, "min_conf": 0.9203612208366394, "min_support": 97, "num_rules": 32}}
```
</details>
