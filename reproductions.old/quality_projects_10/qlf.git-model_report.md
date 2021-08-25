# Model report for file:///tmp/top-repos-quality-repos-o1r3q964/qlf.git HEAD a9c455f7aee41d7901c89ae90dd821c617340a86

### Dump

```json
{'created_at': '2021-08-25 08:13:27',
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
 'size': '20.0 kB',
 'tags': [],
 'uuid': 'b8671c03-e15c-4bf6-8bad-bbc77d299740',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-o1r3q964/qlf.git a9c455f7aee41d7901c89ae90dd821c617340a86

# javascript
42 rules, avg.len. 8.7
## train
PPCR: 0.918033
### report
macro
{'f1-score': 0.7356276366985777,
 'precision': 0.760028980876047,
 'recall': 0.7153809096737389,
 'support': 56515}
micro
{'f1-score': 0.9564186499159515,
 'precision': 0.9564186499159515,
 'recall': 0.9564186499159515,
 'support': 56515}
weighted
{'f1-score': 0.9544484264268077,
 'precision': 0.9539450092898327,
 'recall': 0.9564186499159515,
 'support': 56515}
### report_full
macro
{'f1-score': 0.6849242515993835,
 'precision': 0.760028980876047,
 'recall': 0.6306090807838205,
 'support': 61561}
micro
{'f1-score': 0.9155459195772214,
 'precision': 0.9564186499159515,
 'recall': 0.878023423920989,
 'support': 61561}
weighted
{'f1-score': 0.9102917436373565,
 'precision': 0.9515429209056193,
 'recall': 0.878023423920989,
 'support': 61561}
## test
PPCR: 0.920372
### report
macro
{'f1-score': 0.735311109043913,
 'precision': 0.76055153168743,
 'recall': 0.7137566612470371,
 'support': 13477}
micro
{'f1-score': 0.9525858870668547,
 'precision': 0.9525858870668547,
 'recall': 0.9525858870668547,
 'support': 13477}
weighted
{'f1-score': 0.9513042457819315,
 'precision': 0.9513777151303112,
 'recall': 0.9525858870668547,
 'support': 13477}
### report_full
macro
{'f1-score': 0.6860898908622858,
 'precision': 0.76055153168743,
 'recall': 0.6305637548198021,
 'support': 14643}
micro
{'f1-score': 0.9130867709815079,
 'precision': 0.9525858870668547,
 'recall': 0.87673290992283,
 'support': 14643}
weighted
{'f1-score': 0.9091375627025287,
 'precision': 0.9491732062547152,
 'recall': 0.87673290992283,
 'support': 14643}
```

## javascript
### Summary
29 rules, avg.len. 8.4

| | |
|-|-|
|Min support|95|
|Max support|21563|
|Min confidence|0.9269005656242371|
|Max confidence|0.9991319179534912|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.997. Support: 176.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.998. Support: 2163.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 1723.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.971. Support: 330.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -3.label in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.990. Support: 152.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 1804.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.993. Support: 538.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 1548.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.927. Support: 513.` |
| 10 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = import<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 386.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {SWITCH}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.995. Support: 102.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {SWITCH}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1371.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 286.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 138.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 335.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;}<br>	∧ -1.length ≥ 2<br>	∧ -3.diff_offset ≤ 13<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 313.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = ClassBody<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.996. Support: 126.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = ClassBody<br>⇒ y = ∅<br>Confidence: 0.996. Support: 125.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {), >}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = ClassBody<br>⇒ y = ␣<br>Confidence: 0.984. Support: 765.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IMPORT} and not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {ClassBody}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 483.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {ClassBody}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 233.` |
| 22 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, return}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {IMPORT, KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {ClassBody, File}<br>⇒ y = ⏎<br>Confidence: 0.941. Support: 127.` |
| 23 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION}<br>	∧ ^2.internal_type not in {ClassBody, File}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 95.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {IMPORT, KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {ClassBody, File}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 576.` |
| 25 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {DECLARATION, STATEMENT}<br>	∧ ^2.internal_type not in {ClassBody, File}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 140.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {IMPORT, KEY}<br>	∧ +2.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {ClassBody, File}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1113.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {IMPORT, KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {ClassBody, File}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 21563.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {ClassBody}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 179.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ -2.reserved = =<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type not in {ClassBody}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 179.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.379310344827585, "max_conf": 0.9991319179534912, "max_support": 21563, "min_conf": 0.9269005656242371, "min_support": 95, "num_rules": 29}}
```
</details>
