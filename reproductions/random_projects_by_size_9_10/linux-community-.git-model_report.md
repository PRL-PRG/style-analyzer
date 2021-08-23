# Model report for file:///tmp/top-repos-quality-repos-ys8pl_1m/linux-community-.git HEAD ffd67d7c28195404b593af1e539a82e3d0af13a9

### Dump

```json
{'created_at': '2021-08-20 16:34:05',
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
 'size': '17.0 kB',
 'tags': [],
 'uuid': '04a49609-8e92-4751-894f-662e963a6724',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ys8pl_1m/linux-community-.git ffd67d7c28195404b593af1e539a82e3d0af13a9

# javascript
35 rules, avg.len. 8.0
## train
PPCR: 0.948010
### report
macro
{'f1-score': 0.9322130742571217,
 'precision': 0.9481763876715518,
 'recall': 0.9197868812044674,
 'support': 68178}
micro
{'f1-score': 0.9813869576696295,
 'precision': 0.9813869576696295,
 'recall': 0.9813869576696295,
 'support': 68178}
weighted
{'f1-score': 0.9811173148245451,
 'precision': 0.9812259294011609,
 'recall': 0.9813869576696295,
 'support': 68178}
### report_full
macro
{'f1-score': 0.8634662855999757,
 'precision': 0.9481763876715518,
 'recall': 0.8091376254713449,
 'support': 71917}
micro
{'f1-score': 0.9551946893179628,
 'precision': 0.9813869576696295,
 'recall': 0.9303641698068607,
 'support': 71917}
weighted
{'f1-score': 0.9505309225024263,
 'precision': 0.9792496778555099,
 'recall': 0.9303641698068607,
 'support': 71917}
## test
PPCR: 0.819149
### report
macro
{'f1-score': 0.3131160572337043,
 'precision': 0.4216589861751152,
 'recall': 0.2987179487179487,
 'support': 77}
micro
{'f1-score': 0.6493506493506493,
 'precision': 0.6493506493506493,
 'recall': 0.6493506493506493,
 'support': 77}
weighted
{'f1-score': 0.6498351674822263,
 'precision': 0.7759889879705547,
 'recall': 0.6493506493506493,
 'support': 77}
### report_full
macro
{'f1-score': 0.245715328389379,
 'precision': 0.4216589861751152,
 'recall': 0.21781871510132378,
 'support': 94}
micro
{'f1-score': 0.5847953216374269,
 'precision': 0.6493506493506493,
 'recall': 0.5319148936170213,
 'support': 94}
weighted
{'f1-score': 0.5725540257589813,
 'precision': 0.7721835474066084,
 'recall': 0.5319148936170213,
 'support': 94}
```

## javascript
### Summary
23 rules, avg.len. 7.4

| | |
|-|-|
|Min support|105|
|Max support|13039|
|Min confidence|0.93660968542099|
|Max confidence|0.9998723268508911|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 13039.` |
| 2 | `  -1.label in {<space>}<br>	∧ ^1.roles in {OPERATOR} and not in {ADD, IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.989. Support: 307.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles in {STRING}<br>	∧ ^1.roles in {OPERATOR} and not in {ADD, IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.963. Support: 121.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 205.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 122.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 5168.` |
| 7 | `  -1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.995. Support: 1756.` |
| 8 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 1732.` |
| 9 | `  -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1253.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 208.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 8762.` |
| 12 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 3917.` |
| 13 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 2156.` |
| 14 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, if, }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles in {CALLEE}<br>⇒ y = ⏎⏎<br>Confidence: 0.993. Support: 209.` |
| 15 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, if, }}<br>	∧ +5.roles in {BINARY}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {CALLEE}<br>⇒ y = ⏎<br>Confidence: 0.940. Support: 209.` |
| 16 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 810.` |
| 17 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 111<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 220.` |
| 18 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 110<br>	∧ -2.label not in {<newline>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 111.` |
| 19 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {LEFT}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.937. Support: 702.` |
| 20 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 595.` |
| 21 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 468.` |
| 22 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 7966.` |
| 23 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 105.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.391304347826087, "max_conf": 0.9998723268508911, "max_support": 13039, "min_conf": 0.93660968542099, "min_support": 105, "num_rules": 23}}
```
</details>
