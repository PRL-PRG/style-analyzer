# Model report for file:///tmp/top-repos-quality-repos-5le2datz/wordpress-editor-ios.git HEAD b56b59e8f76b6eaf0bb378b487d7ec38db5a407e

### Dump

```json
{'created_at': '2021-08-29 23:21:54',
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
 'size': '20.6 kB',
 'tags': [],
 'uuid': '4ef14aa9-0311-4d66-a85b-dec2c6279999',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5le2datz/wordpress-editor-ios.git b56b59e8f76b6eaf0bb378b487d7ec38db5a407e

# javascript
127 rules, avg.len. 8.1
## train
PPCR: 0.917053
### report
macro
{'f1-score': 0.6025581012898997,
 'precision': 0.61003749566905,
 'recall': 0.5987964899521266,
 'support': 40597}
micro
{'f1-score': 0.9320639456117445,
 'precision': 0.9320639456117447,
 'recall': 0.9320639456117447,
 'support': 40597}
weighted
{'f1-score': 0.9278177676172297,
 'precision': 0.9244959695215088,
 'recall': 0.9320639456117447,
 'support': 40597}
### report_full
macro
{'f1-score': 0.5305869245382773,
 'precision': 0.61003749566905,
 'recall': 0.4992517957841853,
 'support': 44269}
micro
{'f1-score': 0.8917352060896001,
 'precision': 0.9320639456117447,
 'recall': 0.8547516320675868,
 'support': 44269}
weighted
{'f1-score': 0.8694297737359138,
 'precision': 0.900483990042274,
 'recall': 0.8547516320675868,
 'support': 44269}
## test
PPCR: 0.936733
### report
macro
{'f1-score': 0.546159597981233,
 'precision': 0.5633892253284388,
 'recall': 0.5356578256886478,
 'support': 19307}
micro
{'f1-score': 0.9632257730356866,
 'precision': 0.9632257730356866,
 'recall': 0.9632257730356866,
 'support': 19307}
weighted
{'f1-score': 0.961765875008556,
 'precision': 0.9617909646823372,
 'recall': 0.9632257730356866,
 'support': 19307}
### report_full
macro
{'f1-score': 0.519072282665752,
 'precision': 0.5633892253284388,
 'recall': 0.49579859457334957,
 'support': 20611}
micro
{'f1-score': 0.9317601082218548,
 'precision': 0.9632257730356866,
 'recall': 0.9022851875212266,
 'support': 20611}
weighted
{'f1-score': 0.9144253077156304,
 'precision': 0.9323760191369795,
 'recall': 0.9022851875212266,
 'support': 20611}
```

## javascript
### Summary
55 rules, avg.len. 8.0

| | |
|-|-|
|Min support|130|
|Max support|6437|
|Min confidence|0.9214404821395874|
|Max confidence|0.9996682405471802|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 6394.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 1906.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 1054.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles in {BINARY}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 261.` |
| 5 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 689.` |
| 6 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 5620.` |
| 7 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1482.` |
| 8 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 603.` |
| 9 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 141.` |
| 10 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 178.` |
| 11 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 4752.` |
| 12 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 282.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {(}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 4207.` |
| 14 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≥ 18<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1583.` |
| 15 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.949. Support: 147.` |
| 16 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 213.` |
| 17 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 4977.` |
| 18 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 6437.` |
| 19 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 2030.` |
| 20 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 1024.` |
| 21 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles in {BINARY}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 246.` |
| 22 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 634.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 5511.` |
| 24 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1480.` |
| 25 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.971. Support: 153.` |
| 26 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 27 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 5082.` |
| 28 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.994. Support: 6386.` |
| 29 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 713.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 5540.` |
| 31 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1507.` |
| 32 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.940. Support: 142.` |
| 33 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 4775.` |
| 34 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 5488.` |
| 35 | `  •••start_col ≤ 19<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {BlockStatement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.949. Support: 148.` |
| 36 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 159.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 1947.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 1045.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles in {BINARY}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 252.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {BINARY}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 179.` |
| 41 | `  •••start_col ≥ 37<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≤ 17<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 250.` |
| 42 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.968. Support: 172.` |
| 43 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 541.` |
| 44 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 130.` |
| 45 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 310.` |
| 46 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 4737.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 5493.` |
| 48 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.948. Support: 145.` |
| 49 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 234.` |
| 50 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1468.` |
| 51 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 12<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IF, OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.936. Support: 196.` |
| 52 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.952. Support: 136.` |
| 53 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 5588.` |
| 54 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 322.` |
| 55 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 207.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.018181818181818, "max_conf": 0.9996682405471802, "max_support": 6437, "min_conf": 0.9214404821395874, "min_support": 130, "num_rules": 55}}
```
</details>
