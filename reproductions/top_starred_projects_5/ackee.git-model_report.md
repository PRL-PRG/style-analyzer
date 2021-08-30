# Model report for file:///tmp/top-repos-quality-repos-uau4cre6/ackee.git HEAD 9a6e51d337ab7c9affdc6752e1acbe9405d0f633

### Dump

```json
{'created_at': '2021-08-29 23:11:26',
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
 'size': '16.8 kB',
 'tags': [],
 'uuid': '3d59b73a-fdb3-4412-ab99-0e129580df23',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-uau4cre6/ackee.git 9a6e51d337ab7c9affdc6752e1acbe9405d0f633

# javascript
32 rules, avg.len. 9.1
## train
PPCR: 0.943537
### report
macro
{'f1-score': 0.9323646682883869,
 'precision': 0.9467023700574486,
 'recall': 0.9189454804537297,
 'support': 50466}
micro
{'f1-score': 0.9697221891966868,
 'precision': 0.9697221891966868,
 'recall': 0.9697221891966868,
 'support': 50466}
weighted
{'f1-score': 0.9695317361737019,
 'precision': 0.9695808403160378,
 'recall': 0.9697221891966868,
 'support': 50466}
### report_full
macro
{'f1-score': 0.8468938634820117,
 'precision': 0.9467023700574486,
 'recall': 0.7851105499837342,
 'support': 53486}
micro
{'f1-score': 0.9415499461289827,
 'precision': 0.9697221891966868,
 'recall': 0.9149684029465655,
 'support': 53486}
weighted
{'f1-score': 0.9354290522763157,
 'precision': 0.9674527436487472,
 'recall': 0.9149684029465655,
 'support': 53486}
## test
PPCR: 0.942980
### report
macro
{'f1-score': 0.9205250542302054,
 'precision': 0.9359444418493978,
 'recall': 0.9063655659983088,
 'support': 12734}
micro
{'f1-score': 0.9659180147636249,
 'precision': 0.9659180147636249,
 'recall': 0.9659180147636249,
 'support': 12734}
weighted
{'f1-score': 0.9656393932117423,
 'precision': 0.9656569067679737,
 'recall': 0.9659180147636249,
 'support': 12734}
### report_full
macro
{'f1-score': 0.833875613191499,
 'precision': 0.9359444418493978,
 'recall': 0.7754634080524431,
 'support': 13504}
micro
{'f1-score': 0.9375714612394237,
 'precision': 0.9659180147636249,
 'recall': 0.9108412322274881,
 'support': 13504}
weighted
{'f1-score': 0.9298093573524501,
 'precision': 0.9621553396209886,
 'recall': 0.9108412322274881,
 'support': 13504}
```

## javascript
### Summary
24 rules, avg.len. 8.2

| | |
|-|-|
|Min support|90|
|Max support|10022|
|Min confidence|0.9337301850318909|
|Max confidence|0.9997474551200867|

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
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.986. Support: 5807.` |
| 2 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1091.` |
| 3 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 517.` |
| 4 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 102.` |
| 5 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 7221.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 1980.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 574.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {CALLEE}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2233.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 1339.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1254.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 1251.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.949. Support: 1040.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, }}<br>	∧ +4.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 175.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, }}<br>	∧ +2.reserved = }<br>	∧ +4.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 142.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, }}<br>	∧ +2.reserved not in {}}<br>	∧ +4.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.934. Support: 1260.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.995. Support: 91.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 785.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 600.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {INCOMPLETE} and not in {IDENTIFIER}<br>	∧ +1.reserved = const<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>⇒ y = '<br>Confidence: 0.994. Support: 90.` |
| 20 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type = TemplateElement<br>	∧ +1.reserved not in {), ,, >, const, }}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 209.` |
| 21 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {STRING} and not in {IDENTIFIER}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {TemplateElement}<br>	∧ +1.reserved not in {), ,, const, }}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 111.` |
| 22 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<newline>}<br>	∧ -2.reserved not in {)}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {TemplateElement}<br>	∧ +1.reserved not in {), ,, >, const, }}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 536.` |
| 23 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_offset ≤ 22<br>	∧ -2.reserved not in {), }}<br>	∧ -2.roles not in {PATHNAME}<br>	∧ +1.internal_type not in {TemplateElement}<br>	∧ +1.reserved not in {), ,, :, >, const, }}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 10022.` |
| 24 | `  •••start_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {MODULE}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 121.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.25, "max_conf": 0.9997474551200867, "max_support": 10022, "min_conf": 0.9337301850318909, "min_support": 90, "num_rules": 24}}
```
</details>
