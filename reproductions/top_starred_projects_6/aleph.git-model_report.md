# Model report for file:///tmp/top-repos-quality-repos-vcus7b9x/aleph.git HEAD 42b2afc8ef9c63033c20ff335e70fd8e7b75b1ed

### Dump

```json
{'created_at': '2021-08-29 20:43:04',
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
 'size': '17.6 kB',
 'tags': [],
 'uuid': 'e7afd0b0-a864-4b80-a49d-d207fea3b81b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-vcus7b9x/aleph.git 42b2afc8ef9c63033c20ff335e70fd8e7b75b1ed

# javascript
92 rules, avg.len. 6.5
## train
PPCR: 0.937163
### report
macro
{'f1-score': 0.7353045626641925,
 'precision': 0.8050494106468905,
 'recall': 0.69158616620214,
 'support': 15481}
micro
{'f1-score': 0.9257153930624636,
 'precision': 0.9257153930624636,
 'recall': 0.9257153930624636,
 'support': 15481}
weighted
{'f1-score': 0.9206546561300885,
 'precision': 0.9232167095668941,
 'recall': 0.9257153930624636,
 'support': 15481}
### report_full
macro
{'f1-score': 0.7040665802964731,
 'precision': 0.8050494106468905,
 'recall': 0.653110602212217,
 'support': 16519}
micro
{'f1-score': 0.8956875,
 'precision': 0.9257153930624636,
 'recall': 0.867546461650221,
 'support': 16519}
weighted
{'f1-score': 0.8783895458668846,
 'precision': 0.9063733699689777,
 'recall': 0.867546461650221,
 'support': 16519}
## test
PPCR: 0.949162
### report
macro
{'f1-score': 0.637935104162713,
 'precision': 0.7907759075412892,
 'recall': 0.57449375944127,
 'support': 3510}
micro
{'f1-score': 0.8985754985754986,
 'precision': 0.8985754985754986,
 'recall': 0.8985754985754986,
 'support': 3510}
weighted
{'f1-score': 0.8863714707710104,
 'precision': 0.8986479120689682,
 'recall': 0.8985754985754986,
 'support': 3510}
### report_full
macro
{'f1-score': 0.620861234820695,
 'precision': 0.7907759075412892,
 'recall': 0.5541203889240532,
 'support': 3698}
micro
{'f1-score': 0.8751387347391787,
 'precision': 0.8985754985754986,
 'recall': 0.8528934559221201,
 'support': 3698}
weighted
{'f1-score': 0.8538347408454311,
 'precision': 0.8856609990809905,
 'recall': 0.8528934559221201,
 'support': 3698}
```

## javascript
### Summary
57 rules, avg.len. 5.4

| | |
|-|-|
|Min support|166|
|Max support|3185|
|Min confidence|0.9245689511299133|
|Max confidence|0.9991103410720825|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 110,
                     'min_samples_split': 194,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 1674.` |
| 2 | `  -1.reserved = (<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 570.` |
| 3 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 262.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.reserved not in {)}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 196.` |
| 5 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 562.` |
| 6 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 641.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 257.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 2343.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 193.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 176.` |
| 11 | `  -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 219.` |
| 12 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 236.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 444.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 346.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.998. Support: 221.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.970. Support: 217.` |
| 17 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>⇒ y = '<br>Confidence: 0.998. Support: 326.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 325.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.987. Support: 3185.` |
| 20 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 823.` |
| 21 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 878.` |
| 22 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>⇒ y = ∅<br>Confidence: 0.999. Support: 451.` |
| 23 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.980. Support: 226.` |
| 24 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 270.` |
| 25 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>⇒ y = ∅<br>Confidence: 0.956. Support: 239.` |
| 26 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>⇒ y = ∅<br>Confidence: 0.998. Support: 221.` |
| 27 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 236.` |
| 28 | `  •••start_col ≥ 5<br>	∧ -1.diff_offset ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {)}<br>	∧ -5.diff_offset ≤ 32<br>	∧ +1.reserved not in {), ;, >}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 918.` |
| 29 | `  •••start_col ≥ 5<br>	∧ -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {)}<br>	∧ -5.diff_offset ≤ 32<br>	∧ +1.reserved not in {), ;, >}<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 1082.` |
| 30 | `  •••start_col ≥ 5<br>	∧ -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {)}<br>	∧ -5.diff_offset ≤ 32<br>	∧ +1.reserved not in {), >}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 382.` |
| 31 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 1685.` |
| 32 | `  -1.reserved = (<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 593.` |
| 33 | `  -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.925. Support: 232.` |
| 34 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 235.` |
| 35 | `  +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 545.` |
| 36 | `  +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 601.` |
| 37 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 252.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 2350.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 387.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 355.` |
| 41 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 256.` |
| 42 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 227.` |
| 43 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.956. Support: 236.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 202.` |
| 45 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 794.` |
| 46 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 866.` |
| 47 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.969. Support: 241.` |
| 48 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 273.` |
| 49 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = )<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.978. Support: 202.` |
| 50 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {)}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 239.` |
| 51 | `  -1.diff_offset ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {)}<br>	∧ -5.diff_offset ≤ 34<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 970.` |
| 52 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -5.diff_offset ≤ 34<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 462.` |
| 53 | `  -1.diff_offset ≤ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -5.diff_offset ≤ 34<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 184.` |
| 54 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.956. Support: 259.` |
| 55 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 195.` |
| 56 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.944. Support: 260.` |
| 57 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 166.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.4035087719298245, "max_conf": 0.9991103410720825, "max_support": 3185, "min_conf": 0.9245689511299133, "min_support": 166, "num_rules": 57}}
```
</details>
