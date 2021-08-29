# Model report for file:///tmp/top-repos-quality-repos-7pnoijn4/meteor-files.git HEAD 7f67a0885e120ad77085b191e3399add575a2d89

### Dump

```json
{'created_at': '2021-08-29 16:31:47',
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
 'size': '17.5 kB',
 'tags': [],
 'uuid': '33c5a5c0-7df0-47b4-9d7c-c03e04381b23',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7pnoijn4/meteor-files.git 7f67a0885e120ad77085b191e3399add575a2d89

# javascript
93 rules, avg.len. 7.5
## train
PPCR: 0.986678
### report
macro
{'f1-score': 0.8683074767644948,
 'precision': 0.9271591593723436,
 'recall': 0.8425697448511814,
 'support': 22737}
micro
{'f1-score': 0.9441439064080572,
 'precision': 0.9441439064080573,
 'recall': 0.9441439064080573,
 'support': 22737}
weighted
{'f1-score': 0.9424579201607266,
 'precision': 0.944944127689826,
 'recall': 0.9441439064080573,
 'support': 22737}
### report_full
macro
{'f1-score': 0.8433846446940647,
 'precision': 0.9271591593723436,
 'recall': 0.8079538468770107,
 'support': 23044}
micro
{'f1-score': 0.937812629693541,
 'precision': 0.9441439064080573,
 'recall': 0.9315657003992363,
 'support': 23044}
weighted
{'f1-score': 0.9346049278680936,
 'precision': 0.9447262479584692,
 'recall': 0.9315657003992363,
 'support': 23044}
## test
PPCR: 0.981743
### report
macro
{'f1-score': 0.7865690782507526,
 'precision': 0.8202748012717423,
 'recall': 0.7850094097014918,
 'support': 1667}
micro
{'f1-score': 0.904619076184763,
 'precision': 0.904619076184763,
 'recall': 0.904619076184763,
 'support': 1667}
weighted
{'f1-score': 0.9031763595351707,
 'precision': 0.9063413926377003,
 'recall': 0.904619076184763,
 'support': 1667}
### report_full
macro
{'f1-score': 0.7667205721537745,
 'precision': 0.8202748012717423,
 'recall': 0.7464959497362911,
 'support': 1698}
micro
{'f1-score': 0.8962852897473997,
 'precision': 0.904619076184763,
 'recall': 0.8881036513545347,
 'support': 1698}
weighted
{'f1-score': 0.8936084280925823,
 'precision': 0.9039645674489849,
 'recall': 0.8881036513545347,
 'support': 1698}
```

## javascript
### Summary
61 rules, avg.len. 7.1

| | |
|-|-|
|Min support|145|
|Max support|5575|
|Min confidence|0.9214254021644592|
|Max confidence|0.9996503591537476|

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
                     'min_samples_split': 237,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.991. Support: 5037.` |
| 2 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.993. Support: 377.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 492.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 741.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 212.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 643.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 318.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 386.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 239.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 202.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.length ≤ 3<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FUNCTION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 423.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label in {<space>}<br>	∧ -3.length ≤ 3<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FUNCTION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 180.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 4479.` |
| 14 | `  -1.reserved = ,<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 602.` |
| 15 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.999. Support: 455.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 611.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 357.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 248.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 261.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 5183.` |
| 21 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 517.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 3126.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 154.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1374.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 763.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 369.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.998. Support: 248.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 244.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 157.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1356.` |
| 31 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 4989.` |
| 32 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.981. Support: 385.` |
| 33 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 438.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 3284.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 145.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1408.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 682.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 260.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 363.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 239.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {)}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {FILE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 2259.` |
| 42 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 473.` |
| 43 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 5007.` |
| 44 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.986. Support: 390.` |
| 45 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 501.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 3143.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 169.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1430.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 756.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.998. Support: 274.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.998. Support: 260.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 203.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 601.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 411.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 369.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 269.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 223.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles not in {FILE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 150.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 194.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles in {DECLARATION, FUNCTION} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 205.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 5575.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.131147540983607, "max_conf": 0.9996503591537476, "max_support": 5575, "min_conf": 0.9214254021644592, "min_support": 145, "num_rules": 61}}
```
</details>
