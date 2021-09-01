# Model report for file:///tmp/top-repos-quality-repos-o_j0l67g/react-native.git HEAD 9b4f8e01442356f820e135fae3849063b2b8c92c

### Dump

```json
{'created_at': '2021-08-18 10:37:39',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-80-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.9 (default, Jan 26 2021, 15:33:00) [GCC 8.4.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '27.9 kB',
 'tags': [],
 'uuid': 'bb11f911-b00b-4472-9890-499762540764',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-o_j0l67g/react-native.git 9b4f8e01442356f820e135fae3849063b2b8c92c

# javascript
219 rules, avg.len. 11.1
## train
PPCR: 0.937227
### report
macro
{'f1-score': 0.5550702039700592,
 'precision': 0.5722087988651187,
 'recall': 0.5410821716179649,
 'support': 533677}
micro
{'f1-score': 0.9636877736908279,
 'precision': 0.9636877736908279,
 'recall': 0.9636877736908279,
 'support': 533677}
weighted
{'f1-score': 0.9612301337612549,
 'precision': 0.9601257927613124,
 'recall': 0.9636877736908279,
 'support': 533677}
### report_full
macro
{'f1-score': 0.5110465365066078,
 'precision': 0.5722087988651187,
 'recall': 0.4685829844126317,
 'support': 569421}
micro
{'f1-score': 0.932461123127773,
 'precision': 0.9636877736908279,
 'recall': 0.9031946485991911,
 'support': 569421}
weighted
{'f1-score': 0.9254832225354406,
 'precision': 0.9554566173948411,
 'recall': 0.9031946485991911,
 'support': 569421}
## test
PPCR: 0.933265
### report
macro
{'f1-score': 0.5188277212684308,
 'precision': 0.5265072276408669,
 'recall': 0.5241259913349968,
 'support': 89880}
micro
{'f1-score': 0.953526924788607,
 'precision': 0.953526924788607,
 'recall': 0.953526924788607,
 'support': 89880}
weighted
{'f1-score': 0.9531269650737213,
 'precision': 0.9545725165809111,
 'recall': 0.953526924788607,
 'support': 89880}
### report_full
macro
{'f1-score': 0.47978142188432815,
 'precision': 0.5265072276408669,
 'recall': 0.45918864213188554,
 'support': 96307}
micro
{'f1-score': 0.9206120728085204,
 'precision': 0.953526924788607,
 'recall': 0.889893777191689,
 'support': 96307}
weighted
{'f1-score': 0.9165300340489407,
 'precision': 0.9510214878576162,
 'recall': 0.889893777191689,
 'support': 96307}
```

## javascript
### Summary
219 rules, avg.len. 11.1

| | |
|-|-|
|Min support|90|
|Max support|41249|
|Min confidence|0.802526593208313|
|Max confidence|0.99992436170578|

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
               'min_samples_leaf_max': 130,
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
| 1 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 2921.` |
| 2 | `  •••start_line ≥ 215<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.956. Support: 1428.` |
| 3 | `  •••start_line ≤ 215<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.878. Support: 209.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 1085.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {RIGHT}<br>	∧ -4.diff_offset ≥ 23<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.996. Support: 114.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label in {<space>}<br>	∧ -4.diff_offset ≤ 22<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {STRING}<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.838. Support: 616.` |
| 7 | `  •••start_line ≥ 254<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.877. Support: 727.` |
| 8 | `  •••start_line ≤ 253<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.960. Support: 415.` |
| 9 | `  •••start_col ≤ 40<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = &&<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {CONDITION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.871. Support: 578.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -1.roles in {BINARY}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 781.` |
| 11 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (, {}<br>	∧ -1.roles not in {BINARY}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 137.` |
| 12 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (, {}<br>	∧ -1.roles not in {BINARY}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 241.` |
| 13 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (, {}<br>	∧ -1.roles not in {BINARY}<br>	∧ -3.reserved = =<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 850.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR, OR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 770.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ -4.diff_offset ≥ 8<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +1.length ≤ 13<br>	∧ ^1.roles in {OPERATOR, OR}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 3237.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≤ 7<br>	∧ +1.reserved not in {(, )}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +1.length ≤ 13<br>	∧ ^1.roles in {OPERATOR, OR}<br>⇒ y = ␣<br>Confidence: 0.893. Support: 201.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {BITWISE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 220.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 3<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 200.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 3<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {OPERATOR} and not in {BITWISE, OR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 249.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ -5.length ≤ 7<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {LEFT} and not in {STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.822. Support: 272.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {BITWISE, OPERATOR} and not in {OR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 211.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ -4.diff_offset ≥ 5<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {BITWISE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 2325.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -4.diff_offset ≤ 4<br>	∧ +1.reserved not in {), =}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {BITWISE, OPERATOR} and not in {OR}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 106.` |
| 24 | `  •••start_line ≥ 61<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = &&<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = LogicalExpression<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2969.` |
| 25 | `  •••start_line ≥ 61<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles in {LITERAL} and not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = LogicalExpression<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 129.` |
| 26 | `  •••start_line ≥ 61<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LEFT, LITERAL, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = LogicalExpression<br>	∧ ^1.roles in {OPERATOR} and not in {BITWISE, OR}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.824. Support: 242.` |
| 27 | `  •••start_line ≥ 61<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 41249.` |
| 28 | `  •••start_col ≥ 39<br>	∧ •••start_line ≤ 60<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.892. Support: 162.` |
| 29 | `  •••start_col ≤ 38<br>	∧ •••start_line ≤ 60<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {&&, (}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LEFT, STRING}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.roles not in {FUNCTION}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 1386.` |
| 30 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 6608.` |
| 31 | `  •••start_line = 255<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {EXPRESSION} and not in {BINARY}<br>⇒ y = '<br>Confidence: 0.944. Support: 277.` |
| 32 | `  •••start_line = 255<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = ExpressionStatement<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = '<br>Confidence: 0.804. Support: 196.` |
| 33 | `  •••start_line = 255<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.843. Support: 1331.` |
| 34 | `  •••start_line ≤ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.998. Support: 248.` |
| 35 | `  •••start_line ≤ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≥ 8<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.928. Support: 90.` |
| 36 | `  •••start_line ≤ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≥ 8<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved not in {,, =}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.835. Support: 100.` |
| 37 | `  •••start_line ≤ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.length ≤ 8<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.970. Support: 5042.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.diff_line = 0<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 871.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =}<br>	∧ +2.reserved = (<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.926. Support: 562.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.label not in {<newline>}<br>	∧ -5.diff_offset ≥ 24<br>	∧ +1.reserved not in {), =}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {CONDITION}<br>⇒ y = ␣<br>Confidence: 0.864. Support: 92.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.label not in {<newline>}<br>	∧ -5.diff_offset ≤ 23<br>	∧ +1.reserved = :<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {CONDITION}<br>⇒ y = ␣<br>Confidence: 0.857. Support: 381.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.label not in {<newline>}<br>	∧ -5.diff_offset ≤ 23<br>	∧ -5.length ≤ 2<br>	∧ +1.reserved not in {), :, =}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {CONDITION}<br>⇒ y = ␣<br>Confidence: 0.909. Support: 160.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 313.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 5<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 109.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 5<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 606.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 4<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 821.` |
| 47 | `  -1.internal_type = DirectiveLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.969. Support: 207.` |
| 48 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.953. Support: 1363.` |
| 49 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_line = 0<br>	∧ -4.label in {<space>}<br>	∧ -5.reserved = ,<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 256.` |
| 50 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved = .<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved = ,<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 131.` |
| 51 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.reserved not in {.}<br>	∧ -3.diff_line = 0<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved = ,<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 102.` |
| 52 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.diff_line = 0<br>	∧ -5.reserved not in {,}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 17055.` |
| 53 | `  •••start_line ≥ 254<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 131.` |
| 54 | `  •••start_col ≤ 42<br>	∧ •••start_line ≥ 254<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.867. Support: 281.` |
| 55 | `  •••start_line ≤ 254<br>	∧ -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 1767.` |
| 56 | `  -1.internal_type not in {DirectiveLiteral, StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 21741.` |
| 57 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.957. Support: 200.` |
| 58 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 31889.` |
| 59 | `  •••start_line = 255<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 61<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.961. Support: 320.` |
| 60 | `  •••start_line = 255<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≤ 61<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.995. Support: 93.` |
| 61 | `  •••start_line = 255<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved = throw<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≤ 61<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = "<br>Confidence: 0.873. Support: 122.` |
| 62 | `  •••start_line = 255<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≤ 14<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = "<br>Confidence: 0.851. Support: 400.` |
| 63 | `  •••start_line ≤ 254<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.908. Support: 2050.` |
| 64 | `  •••start_col ≥ 48<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ +3.length ≥ 7<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.889. Support: 373.` |
| 65 | `  •••start_col ≥ 48<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 6<br>	∧ +3.length ≤ 6<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.904. Support: 140.` |
| 66 | `  •••start_col ≤ 47<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≤ 10<br>	∧ +1.roles not in {STRING}<br>	∧ +3.length ≥ 14<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.839. Support: 146.` |
| 67 | `  •••start_col ≤ 47<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 34<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 13<br>	∧ +3.length ≤ 13<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.817. Support: 90.` |
| 68 | `  •••start_col ≤ 47<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 34<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 13<br>	∧ +3.length ≤ 13<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.821. Support: 266.` |
| 69 | `  •••start_col ≤ 47<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 12<br>	∧ +3.length ≤ 13<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.875. Support: 1426.` |
| 70 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 1<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.976. Support: 225.` |
| 71 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 6<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.903. Support: 304.` |
| 72 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {if}<br>	∧ -3.length ≤ 5<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 741.` |
| 73 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ +4.reserved not in {,}<br>	∧ +5.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {ASSIGNMENT} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.910. Support: 706.` |
| 74 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 5<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ +3.reserved not in {=}<br>	∧ +3.length ≤ 24<br>	∧ +4.reserved not in {,}<br>	∧ +5.roles in {EXPRESSION}<br>	∧ ^1.roles not in {ASSIGNMENT, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.888. Support: 201.` |
| 75 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ +3.reserved not in {=}<br>	∧ +3.length ≤ 24<br>	∧ +4.reserved not in {,}<br>	∧ +5.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {ASSIGNMENT, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 678.` |
| 76 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 2<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ +3.reserved not in {=}<br>	∧ +3.length ≤ 12<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {ASSIGNMENT, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 7545.` |
| 77 | `  •••start_col ≥ 70<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≤ 1<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {ASSIGNMENT, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 143.` |
| 78 | `  •••start_col ≤ 69<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≤ 1<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {ASSIGNMENT, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 13994.` |
| 79 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.834. Support: 287.` |
| 80 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -5.length ≥ 3<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {}}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.879. Support: 648.` |
| 81 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -5.length ≥ 3<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {}}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.845. Support: 184.` |
| 82 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.986. Support: 7583.` |
| 83 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 1196.` |
| 84 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -5.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 94.` |
| 85 | `  •••start_col ≥ 41<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≥ 10<br>	∧ -5.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.813. Support: 99.` |
| 86 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≤ 9<br>	∧ -5.roles not in {MAP}<br>	∧ -5.length ≥ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.976. Support: 228.` |
| 87 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = SwitchStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.982. Support: 767.` |
| 88 | `  •••start_col ≤ 48<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 5<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.908. Support: 277.` |
| 89 | `  •••start_col ≥ 15<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {ASSIGNMENT}<br>	∧ -3.reserved not in {)}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ForStatement, IfStatement, SwitchStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 2334.` |
| 90 | `  •••start_col ≥ 54<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ASSIGNMENT}<br>	∧ -3.reserved not in {)}<br>	∧ -3.roles in {ASSIGNMENT}<br>	∧ -5.diff_offset ≤ 49<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {IfStatement, SwitchStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.823. Support: 144.` |
| 91 | `  •••start_col ≥ 15<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ASSIGNMENT}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {)}<br>	∧ -3.roles not in {ASSIGNMENT}<br>	∧ -5.diff_offset ≤ 49<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {IfStatement, SwitchStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.851. Support: 5388.` |
| 92 | `  •••start_col ≤ 30<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {FUNCTION} and not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.817. Support: 90.` |
| 93 | `  •••start_col ≥ 15<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = FunctionExpression<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.813. Support: 1071.` |
| 94 | `  •••start_col ≥ 15<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = import<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.882. Support: 487.` |
| 95 | `  •••start_col ≥ 15<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles in {IMPORT}<br>	∧ +1.reserved not in {import}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.923. Support: 175.` |
| 96 | `  •••start_col ≥ 15<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles not in {IMPORT}<br>	∧ -5.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.803. Support: 752.` |
| 97 | `  •••start_col ≤ 14<br>	∧ •••start_line ≥ 254<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label in {"}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {SwitchStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.974. Support: 209.` |
| 98 | `  •••start_col ≤ 14<br>	∧ •••start_line ≥ 254<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ -4.label not in {"}<br>	∧ -5.length ≥ 6<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = FunctionDeclaration<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.835. Support: 118.` |
| 99 | `  •••start_col ≤ 14<br>	∧ •••start_line ≥ 254<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ -4.label not in {"}<br>	∧ -5.length ≤ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.806. Support: 478.` |
| 100 | `  •••start_col ≤ 14<br>	∧ •••start_line ≥ 254<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line = 0<br>	∧ -4.label not in {"}<br>	∧ -5.length ≤ 2<br>	∧ +1.length ≥ 2<br>	∧ +3.length ≥ 10<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.923. Support: 111.` |
| 101 | `  •••start_col ≤ 14<br>	∧ •••start_line ≤ 253<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.830. Support: 362.` |
| 102 | `  •••start_col ≤ 14<br>	∧ •••start_line ≤ 253<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, SwitchStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.860. Support: 752.` |
| 103 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 3<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 4490.` |
| 104 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 3<br>	∧ -5.diff_line = 0<br>	∧ -5.length ≥ 5<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.906. Support: 112.` |
| 105 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.length ≥ 3<br>	∧ -5.diff_line = 0<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ +4.reserved = ,<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.888. Support: 934.` |
| 106 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 5533.` |
| 107 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ +1.length ≥ 7<br>	∧ +2.reserved = :<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 2316.` |
| 108 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ +1.length ≤ 6<br>	∧ +2.reserved = :<br>	∧ +4.reserved = ,<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ⏎<br>Confidence: 0.885. Support: 553.` |
| 109 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ -5.diff_offset ≤ 11<br>	∧ +1.length ≤ 6<br>	∧ +2.reserved = :<br>	∧ +3.roles in {MAP}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 106.` |
| 110 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ +1.length ≤ 6<br>	∧ +2.reserved = :<br>	∧ +3.roles not in {MAP}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ArrayExpression}<br>⇒ y = ⏎<br>Confidence: 0.827. Support: 228.` |
| 111 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ +1.length ≤ 7<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.827. Support: 147.` |
| 112 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ -5.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {:, =}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.901. Support: 570.` |
| 113 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {EXPRESSION} and not in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ -5.label not in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {:, =}<br>	∧ ^1.roles in {LIST} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 410.` |
| 114 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {EXPRESSION} and not in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ -5.label not in {<newline>}<br>	∧ +1.length ≤ 10<br>	∧ +2.reserved not in {:, =}<br>	∧ ^1.roles not in {LIST, LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.821. Support: 792.` |
| 115 | `  •••start_line ≥ 254<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.912. Support: 356.` |
| 116 | `  •••start_line ≤ 253<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.963. Support: 2209.` |
| 117 | `  •••start_line ≥ 254<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = ThrowStatement<br>⇒ y = "<br>Confidence: 0.923. Support: 97.` |
| 118 | `  •••start_line ≥ 254<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.969. Support: 532.` |
| 119 | `  •••start_line ≤ 254<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 8<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.808. Support: 91.` |
| 120 | `  •••start_line ≤ 254<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 8<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.852. Support: 91.` |
| 121 | `  •••start_line ≤ 254<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 8<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.954. Support: 639.` |
| 122 | `  •••start_line ≥ 253<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.805. Support: 752.` |
| 123 | `  •••start_line ≤ 253<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = Identifier<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.819. Support: 240.` |
| 124 | `  •••start_line ≥ 13<br>	∧ -1.diff_col ≥ 10<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 565.` |
| 125 | `  •••start_line ≤ 12<br>	∧ -1.diff_col ≥ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 205.` |
| 126 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {CONDITION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.987. Support: 510.` |
| 127 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.998. Support: 283.` |
| 128 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 352.` |
| 129 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<space>}<br>	∧ -3.length ≤ 3<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.855. Support: 577.` |
| 130 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.898. Support: 221.` |
| 131 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved = (<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.814. Support: 486.` |
| 132 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -4.reserved = :<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 224.` |
| 133 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {:}<br>	∧ -5.diff_offset ≥ 19<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.909. Support: 149.` |
| 134 | `  -1.diff_col ≤ 10<br>	∧ -1.diff_offset ≥ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved = ,<br>	∧ -4.reserved not in {:}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.808. Support: 91.` |
| 135 | `  -1.diff_col ≤ 10<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved = ,<br>	∧ -4.reserved not in {:}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.809. Support: 165.` |
| 136 | `  •••start_col ≤ 13<br>	∧ -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {:}<br>	∧ +1.roles in {ASSIGNMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 101.` |
| 137 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {,}<br>	∧ -4.reserved not in {:}<br>	∧ +1.roles in {EXPRESSION} and not in {ASSIGNMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 25148.` |
| 138 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.reserved not in {:}<br>	∧ -4.reserved not in {:}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 443.` |
| 139 | `  -1.diff_col ≤ 10<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.reserved not in {:}<br>	∧ -4.reserved not in {:}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 235.` |
| 140 | `  -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2726.` |
| 141 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.893. Support: 528.` |
| 142 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = Program<br>⇒ y = ␣<br>Confidence: 0.998. Support: 273.` |
| 143 | `  •••start_line ≥ 231<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {else}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.roles in {BLOCK} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.876. Support: 190.` |
| 144 | `  •••start_line ≥ 231<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {else}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {BLOCK, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.902. Support: 107.` |
| 145 | `  •••start_line ≥ 231<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {else}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 2114.` |
| 146 | `  •••start_col ≥ 7<br>	∧ •••start_line ≤ 230<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {else}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.861. Support: 506.` |
| 147 | `  •••start_col ≤ 7<br>	∧ •••start_line ≤ 230<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 7<br>	∧ +1.reserved not in {else}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {BODY, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎⏎<br>Confidence: 0.848. Support: 115.` |
| 148 | `  •••start_col ≤ 7<br>	∧ •••start_line ≤ 230<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {else}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {BODY, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎⏎<br>Confidence: 0.955. Support: 165.` |
| 149 | `  -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 12<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1794.` |
| 150 | `  -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ +2.roles in {CASE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 616.` |
| 151 | `  -1.reserved not in {(, ,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ +2.roles in {CASE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 362.` |
| 152 | `  -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ +2.roles not in {CASE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 217.` |
| 153 | `  -1.reserved not in {(, ,, ;, =, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.reserved = if<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.895. Support: 100.` |
| 154 | `  -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {CASE} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.999. Support: 807.` |
| 155 | `  -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {IDENTIFIER} and not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ +2.roles not in {CASE}<br>	∧ ^1.roles not in {CASE, OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 97.` |
| 156 | `  -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = throw<br>	∧ +1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.length ≤ 11<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.960. Support: 410.` |
| 157 | `  -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.length ≤ 11<br>	∧ +2.reserved = (<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.985. Support: 494.` |
| 158 | `  -1.reserved not in {(, ), ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {throw}<br>	∧ +1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved = (<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {CASE, OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 129.` |
| 159 | `  -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>} and not in {<space>}<br>	∧ +1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.length ≤ 11<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.937. Support: 119.` |
| 160 | `  •••start_col ≥ 66<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>, <space>}<br>	∧ +1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.length ≤ 11<br>	∧ +2.reserved not in {(}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.855. Support: 93.` |
| 161 | `  •••start_col ≤ 65<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>, <space>}<br>	∧ +1.reserved not in {throw}<br>	∧ +1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.length ≤ 11<br>	∧ +2.roles not in {CALL}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {CASE, OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.856. Support: 447.` |
| 162 | `  -1.diff_col ≥ 25<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.853. Support: 234.` |
| 163 | `  -1.diff_col ≥ 25<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≤ 5<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.997. Support: 146.` |
| 164 | `  -1.diff_col ≤ 24<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 175.` |
| 165 | `  -1.diff_col ≤ 24<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {:}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ +2.roles in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 165.` |
| 166 | `  -1.diff_col ≤ 24<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {:}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = '<br>Confidence: 0.995. Support: 92.` |
| 167 | `  -1.diff_col ≤ 24<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {:}<br>	∧ -4.reserved = :<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.864. Support: 92.` |
| 168 | `  -1.diff_col ≤ 24<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {:}<br>	∧ -4.reserved not in {:}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1025.` |
| 169 | `  •••start_line ≤ 204<br>	∧ -1.diff_col ≤ 24<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, :, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≥ 4<br>	∧ -3.reserved not in {:}<br>	∧ -4.reserved not in {:}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.821. Support: 176.` |
| 170 | `  -1.diff_col ≤ 7<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 3<br>	∧ -3.reserved not in {:}<br>	∧ -4.reserved not in {:}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 11<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.858. Support: 1451.` |
| 171 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎<br>Confidence: 0.963. Support: 287.` |
| 172 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 2<br>	∧ -4.label not in {<newline>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 2<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 363.` |
| 173 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≤ 1<br>	∧ -4.label not in {<newline>}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 2<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 724.` |
| 174 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 201.` |
| 175 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = ,<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 123.` |
| 176 | `  -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {), ,}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.810. Support: 113.` |
| 177 | `  •••start_line ≥ 254<br>	∧ -1.reserved not in {(, )}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {), ,}<br>	∧ -3.length ≤ 12<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.860. Support: 4797.` |
| 178 | `  •••start_line ≤ 253<br>	∧ -1.reserved not in {(, )}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {), ,}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.956. Support: 3719.` |
| 179 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 14449.` |
| 180 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 8836.` |
| 181 | `  -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 649.` |
| 182 | `  -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 409.` |
| 183 | `  -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 128.` |
| 184 | `  -1.internal_type = StringLiteralTypeAnnotation<br>	∧ -1.reserved not in {(, ), :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = UnionTypeAnnotation<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.995. Support: 104.` |
| 185 | `  -1.reserved not in {(, ), ,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE, STATEMENT} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.840. Support: 147.` |
| 186 | `  -1.reserved not in {(, ), ,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ +3.length ≥ 7<br>	∧ ^1.roles in {INCOMPLETE} and not in {BLOCK, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 966.` |
| 187 | `  -1.reserved not in {(, ), ,, :}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {=}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≤ 6<br>	∧ ^1.roles in {INCOMPLETE} and not in {BLOCK, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 6878.` |
| 188 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.856. Support: 484.` |
| 189 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.906. Support: 527.` |
| 190 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = StringLiteral<br>	∧ -5.reserved = +<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.940. Support: 126.` |
| 191 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = StringLiteral<br>	∧ -5.reserved not in {+}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.972. Support: 270.` |
| 192 | `  -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.roles in {CALL}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 266.` |
| 193 | `  -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = NumericLiteral<br>	∧ -2.roles not in {CALL}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.919. Support: 241.` |
| 194 | `  -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.reserved = ,<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.824. Support: 111.` |
| 195 | `  -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.reserved = ,<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 291.` |
| 196 | `  -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.reserved not in {,}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 4292.` |
| 197 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 1918.` |
| 198 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = )<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 246.` |
| 199 | `  •••start_col ≤ 60<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = )<br>	∧ -3.diff_col ≥ 6<br>	∧ -5.length ≤ 12<br>	∧ +1.reserved not in {), ,, :, ;, >, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {BODY, THEN} and not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ⏎<br>Confidence: 0.864. Support: 187.` |
| 200 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = )<br>	∧ +1.reserved not in {), ,, :, ;, >, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BODY, INCOMPLETE, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ⏎<br>Confidence: 0.839. Support: 1427.` |
| 201 | `  -1.reserved = function<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 640.` |
| 202 | `  -1.reserved not in {(, function}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label in {<space>}<br>	∧ +1.reserved = :<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 307.` |
| 203 | `  -1.reserved not in {(, function}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = :<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 577.` |
| 204 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.995. Support: 92.` |
| 205 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.918. Support: 261.` |
| 206 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {), }}<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.863. Support: 252.` |
| 207 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {), }}<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.844. Support: 395.` |
| 208 | `  -1.reserved not in {(, ;, function, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {), }}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 417.` |
| 209 | `  -1.reserved not in {(, ;, function, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.905. Support: 702.` |
| 210 | `  -1.reserved not in {(, ;, function, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +1.roles in {LITERAL, STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.857. Support: 136.` |
| 211 | `  -1.reserved not in {(, ;, function, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {)}<br>	∧ -3.internal_type = Identifier<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles in {LITERAL} and not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.867. Support: 154.` |
| 212 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {), }}<br>	∧ -3.internal_type not in {Identifier}<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.916. Support: 2691.` |
| 213 | `  -1.reserved not in {(, ;, function, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {), ,, :, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.833. Support: 117.` |
| 214 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {), }}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved not in {), ,, :, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {}}<br>	∧ ^1.roles in {DECLARATION} and not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 149.` |
| 215 | `  -1.reserved not in {(, ;, function, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 8<br>	∧ -2.reserved not in {), }}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {}}<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.876. Support: 93.` |
| 216 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved not in {)}<br>	∧ -3.label in {<newline>}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved not in {), ,, :}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.928. Support: 132.` |
| 217 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved not in {)}<br>	∧ -3.label not in {<newline>}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved not in {), ,, :}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.883. Support: 1382.` |
| 218 | `  -1.reserved not in {(, ,, ;, function, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved not in {), }}<br>	∧ -4.reserved not in {=}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {}}<br>	∧ +2.roles in {ASSIGNMENT}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 1531.` |
| 219 | `  -1.reserved not in {(, ,, ;, function, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 7<br>	∧ -2.reserved not in {), }}<br>	∧ -4.reserved not in {=}<br>	∧ -4.roles not in {KEY}<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {}}<br>	∧ +2.roles not in {ASSIGNMENT}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 17605.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 11.100456621004566, "max_conf": 0.99992436170578, "max_support": 41249, "min_conf": 0.802526593208313, "min_support": 90, "num_rules": 219}}
```
</details>
