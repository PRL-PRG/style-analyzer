# Model report for file:///tmp/top-repos-quality-repos-5jk3sdmg/cnn-explainer.git HEAD 09342105af93e2d363871b600f3612a871406728

### Dump

```json
{'created_at': '2021-08-30 03:59:10',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.7 kB',
 'tags': [],
 'uuid': '0ee0bf0d-612c-4bf9-a3d6-62d8c1e34617',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5jk3sdmg/cnn-explainer.git 09342105af93e2d363871b600f3612a871406728

# javascript
109 rules, avg.len. 7.6
## train
PPCR: 0.943816
### report
macro
{'f1-score': 0.6079866396137282,
 'precision': 0.6567913125937193,
 'recall': 0.5806174950927112,
 'support': 32522}
micro
{'f1-score': 0.9409630404034193,
 'precision': 0.9409630404034193,
 'recall': 0.9409630404034193,
 'support': 32522}
weighted
{'f1-score': 0.9350546658367574,
 'precision': 0.9314847365118835,
 'recall': 0.9409630404034193,
 'support': 32522}
### report_full
macro
{'f1-score': 0.5573848653799099,
 'precision': 0.6567913125937193,
 'recall': 0.5146341806815513,
 'support': 34458}
micro
{'f1-score': 0.9137653030755449,
 'precision': 0.9409630404034193,
 'recall': 0.8880956526786232,
 'support': 34458}
weighted
{'f1-score': 0.8963920852189726,
 'precision': 0.9164681175215469,
 'recall': 0.8880956526786232,
 'support': 34458}
## test
PPCR: 0.950553
### report
macro
{'f1-score': 0.6563547837336847,
 'precision': 0.6817146082376274,
 'recall': 0.643728947029574,
 'support': 3095}
micro
{'f1-score': 0.9273021001615509,
 'precision': 0.9273021001615509,
 'recall': 0.9273021001615509,
 'support': 3095}
weighted
{'f1-score': 0.9209838796023849,
 'precision': 0.9171285928647585,
 'recall': 0.9273021001615509,
 'support': 3095}
### report_full
macro
{'f1-score': 0.6136932518793121,
 'precision': 0.6817146082376274,
 'recall': 0.5790617307276205,
 'support': 3256}
micro
{'f1-score': 0.9037946780034639,
 'precision': 0.9273021001615509,
 'recall': 0.8814496314496314,
 'support': 3256}
weighted
{'f1-score': 0.8897599221193826,
 'precision': 0.9074469532099049,
 'recall': 0.8814496314496314,
 'support': 3256}
```

## javascript
### Summary
63 rules, avg.len. 6.7

| | |
|-|-|
|Min support|140|
|Max support|7041|
|Min confidence|0.9266666769981384|
|Max confidence|0.9997791647911072|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 1.000. Support: 1536.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {RIGHT}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 2337.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 290.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.932. Support: 1676.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 1239.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 7013.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2196.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 575.` |
| 9 | `  -1.diff_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 790.` |
| 10 | `  -3.roles in {BINARY}<br>	∧ +3.roles in {RIGHT}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 225.` |
| 11 | `  +3.roles not in {RIGHT}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 2237.` |
| 12 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 1.000. Support: 1477.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 708.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 7041.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 3140.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 962.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 680.` |
| 18 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 352.` |
| 19 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 399.` |
| 20 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 186.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.995. Support: 1272.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 7017.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2264.` |
| 24 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, .}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 804.` |
| 25 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 521.` |
| 26 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 171.` |
| 27 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, .}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 828.` |
| 28 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 498.` |
| 29 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 189.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.roles in {BINARY}<br>	∧ +3.roles in {RIGHT}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 216.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.991. Support: 1356.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 3211.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 919.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 657.` |
| 35 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 343.` |
| 36 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 305.` |
| 37 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 191.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type = Identifier<br>	∧ ^1.internal_type = ArrowFunctionExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 140.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {ArrowFunctionExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 6650.` |
| 40 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, .}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 829.` |
| 41 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 593.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.993. Support: 1236.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ ^1.internal_type = ArrowFunctionExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 140.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {ArrowFunctionExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 6706.` |
| 45 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 515.` |
| 46 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ]<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 152.` |
| 47 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 159.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.994. Support: 1252.` |
| 49 | `  -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, .}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 851.` |
| 50 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 181.` |
| 51 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +4.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 588.` |
| 52 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 3337.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.994. Support: 1316.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type = Identifier<br>	∧ ^1.roles in {DECLARATION} and not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 273.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 6449.` |
| 56 | `  -1.diff_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, .}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 789.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.972. Support: 264.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type = CallExpression<br>⇒ y = '<br>Confidence: 1.000. Support: 1379.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 167.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {RIGHT}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 2335.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, .}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 786.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 570.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ., ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 172.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.666666666666667, "max_conf": 0.9997791647911072, "max_support": 7041, "min_conf": 0.9266666769981384, "min_support": 140, "num_rules": 63}}
```
</details>
