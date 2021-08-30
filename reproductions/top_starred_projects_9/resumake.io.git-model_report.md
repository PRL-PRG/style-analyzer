# Model report for file:///tmp/top-repos-quality-repos-ck818s4y/resumake.io.git HEAD 9f52bcc874b54e1449b9225ab9a117678238d01f

### Dump

```json
{'created_at': '2021-08-29 12:41:09',
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
 'size': '22.3 kB',
 'tags': [],
 'uuid': 'a7f3b776-eac8-4f8c-8931-8fbc4ec14ef1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ck818s4y/resumake.io.git 9f52bcc874b54e1449b9225ab9a117678238d01f

# javascript
118 rules, avg.len. 9.7
## train
PPCR: 0.937887
### report
macro
{'f1-score': 0.551027569307882,
 'precision': 0.5645755198079707,
 'recall': 0.5402313578429225,
 'support': 28765}
micro
{'f1-score': 0.9022423083608552,
 'precision': 0.9022423083608552,
 'recall': 0.9022423083608552,
 'support': 28765}
weighted
{'f1-score': 0.89569421624646,
 'precision': 0.8910001229697521,
 'recall': 0.9022423083608552,
 'support': 28765}
### report_full
macro
{'f1-score': 0.5246550381205097,
 'precision': 0.5645755198079707,
 'recall': 0.49391704545050286,
 'support': 30670}
micro
{'f1-score': 0.8733237991082695,
 'precision': 0.9022423083608552,
 'recall': 0.8462014998369742,
 'support': 30670}
weighted
{'f1-score': 0.8639862531604038,
 'precision': 0.8862552700038335,
 'recall': 0.8462014998369742,
 'support': 30670}
## test
PPCR: 0.947166
### report
macro
{'f1-score': 0.5413079127263111,
 'precision': 0.5479355144527934,
 'recall': 0.5363964788532564,
 'support': 7637}
micro
{'f1-score': 0.8909257561869844,
 'precision': 0.8909257561869844,
 'recall': 0.8909257561869844,
 'support': 7637}
weighted
{'f1-score': 0.8895529627909935,
 'precision': 0.8907147202725251,
 'recall': 0.8909257561869844,
 'support': 7637}
### report_full
macro
{'f1-score': 0.523873404230704,
 'precision': 0.5479355144527934,
 'recall': 0.5037215459614488,
 'support': 8063}
micro
{'f1-score': 0.866751592356688,
 'precision': 0.8909257561869844,
 'recall': 0.8438546446731986,
 'support': 8063}
weighted
{'f1-score': 0.8632939480686385,
 'precision': 0.8862493233628511,
 'recall': 0.8438546446731986,
 'support': 8063}
```

## javascript
### Summary
65 rules, avg.len. 8.9

| | |
|-|-|
|Min support|131|
|Max support|4952|
|Min confidence|0.9233619570732117|
|Max confidence|0.9994861483573914|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>⇒ y = '<br>Confidence: 0.999. Support: 889.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 494.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 322.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +3.roles in {STRING}<br>	∧ ^1.roles in {IMPORT} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 161.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {IMPORT, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 4708.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 263.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {QUALIFIED} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 951.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 855.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {IMPORT}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 144.` |
| 10 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 201.` |
| 11 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 175.` |
| 12 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>, }}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 365.` |
| 13 | `  •••start_col ≤ 11<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>, }}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.958. Support: 177.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION, IMPORT}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 148.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {IMPORT}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 4729.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {IDENTIFIER} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 912.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 905.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {IMPORT}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 151.` |
| 19 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {'} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 190.` |
| 20 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {', <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 21 | `  •••start_col ≥ 6<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {', <space>}<br>	∧ -1.reserved not in {(, `, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {>, return, }}<br>	∧ +4.reserved not in {return}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER, INCOMPLETE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 376.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 290.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {JSXOpeningElement, UnionTypeAnnotation}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 4952.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ ^1.roles in {IDENTIFIER} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 1645.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 460.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 353.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {DECLARATION} and not in {IDENTIFIER, INCOMPLETE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 225.` |
| 28 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = '<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 199.` |
| 29 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {', (, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 192.` |
| 30 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {', (, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved not in {>, }}<br>	∧ ^1.roles in {FUNCTION} and not in {IDENTIFIER, INCOMPLETE, LITERAL}<br>⇒ y = ⏎⏎<br>Confidence: 0.930. Support: 164.` |
| 31 | `  •••start_col ≤ 39<br>	∧ -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {', (, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {>, }}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, LITERAL, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 1953.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -3.label in {<newline>}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 131.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :}<br>	∧ -4.reserved not in {=}<br>	∧ -5.diff_offset ≥ 23<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {JSXOpeningElement, UnionTypeAnnotation}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 177.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {JSXOpeningElement, UnionTypeAnnotation}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 4549.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ ^1.roles in {QUALIFIED} and not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 1619.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 448.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 310.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +2.roles not in {IMPORT}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 2564.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = Identifier<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 608.` |
| 40 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {'} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 213.` |
| 41 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 973.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 932.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 912.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {IMPORT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 144.` |
| 45 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.reserved = import<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 208.` |
| 46 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 184.` |
| 47 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, `, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>, return, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 394.` |
| 48 | `  •••start_col ≤ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, `, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 1<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>, return, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE, LITERAL}<br>⇒ y = ⏎⏎<br>Confidence: 0.954. Support: 142.` |
| 49 | `  •••start_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>	∧ ^2.internal_type = File<br>⇒ y = ⏎⏎<br>Confidence: 0.952. Support: 219.` |
| 50 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {'} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = import<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 219.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION} and not in {IMPORT}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 298.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {IDENTIFIER} and not in {LITERAL}<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {JSXOpeningElement, UnionTypeAnnotation}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 145.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement, UnionTypeAnnotation}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 3012.` |
| 54 | `  •••start_col ≤ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 148.` |
| 55 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = '<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 206.` |
| 56 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {', (, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 182.` |
| 57 | `  •••start_col ≥ 5<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {', (, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {>, }}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 417.` |
| 58 | `  •••start_col ≤ 11<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {', (, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {>, }}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.960. Support: 213.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {IMPORT} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 167.` |
| 60 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 201.` |
| 61 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {INCOMPLETE}<br>⇒ y = ⏎<br>Confidence: 0.935. Support: 333.` |
| 62 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {'} and not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = import<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 183.` |
| 63 | `  •••start_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {', <space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 187.` |
| 64 | `  •••start_col ≤ 6<br>	∧ -1.diff_offset ≥ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 161.` |
| 65 | `  •••start_col ≤ 6<br>	∧ -1.diff_offset ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ⏎⏎<br>Confidence: 0.942. Support: 200.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.938461538461539, "max_conf": 0.9994861483573914, "max_support": 4952, "min_conf": 0.9233619570732117, "min_support": 131, "num_rules": 65}}
```
</details>
