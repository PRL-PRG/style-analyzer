# Model report for file:///tmp/top-repos-quality-repos-5uy1ba_y/radare2-r2pipe.git HEAD 4e8baade815fdbbac359e3f8d185287e761d7f97

### Dump

```json
{'created_at': '2021-08-21 20:42:17',
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
 'size': '17.6 kB',
 'tags': [],
 'uuid': '812b18a7-05f0-4571-a17a-142f45617874',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5uy1ba_y/radare2-r2pipe.git 4e8baade815fdbbac359e3f8d185287e761d7f97

# javascript
105 rules, avg.len. 6.0
## train
PPCR: 0.974224
### report
macro
{'f1-score': 0.7801673519502316,
 'precision': 0.7939429323785799,
 'recall': 0.7689080899832204,
 'support': 10394}
micro
{'f1-score': 0.9335193380796614,
 'precision': 0.9335193380796614,
 'recall': 0.9335193380796614,
 'support': 10394}
weighted
{'f1-score': 0.9275879589634675,
 'precision': 0.9231625968108015,
 'recall': 0.9335193380796614,
 'support': 10394}
### report_full
macro
{'f1-score': 0.7710344045672223,
 'precision': 0.7939429323785799,
 'recall': 0.7528752906503051,
 'support': 10669}
micro
{'f1-score': 0.9213312443621517,
 'precision': 0.9335193380796614,
 'recall': 0.9094573062142657,
 'support': 10669}
weighted
{'f1-score': 0.9126573415430784,
 'precision': 0.9181868479206236,
 'recall': 0.9094573062142657,
 'support': 10669}
## test
PPCR: 0.981698
### report
macro
{'f1-score': 0.7758985676057782,
 'precision': 0.7796196282404015,
 'recall': 0.7738743560518409,
 'support': 2682}
micro
{'f1-score': 0.9146159582401193,
 'precision': 0.9146159582401193,
 'recall': 0.9146159582401193,
 'support': 2682}
weighted
{'f1-score': 0.9056888529311892,
 'precision': 0.8986161147104218,
 'recall': 0.9146159582401193,
 'support': 2682}
### report_full
macro
{'f1-score': 0.7703830457818286,
 'precision': 0.7796196282404015,
 'recall': 0.7637259657543157,
 'support': 2732}
micro
{'f1-score': 0.9061691909863318,
 'precision': 0.9146159582401193,
 'recall': 0.8978770131771596,
 'support': 2732}
weighted
{'f1-score': 0.8946689430845247,
 'precision': 0.8936880809069875,
 'recall': 0.8978770131771596,
 'support': 2732}
```

## javascript
### Summary
77 rules, avg.len. 5.2

| | |
|-|-|
|Min support|143|
|Max support|1480|
|Min confidence|0.9276253581047058|
|Max confidence|0.9992917776107788|

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
                     'min_samples_leaf': 91,
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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 1224.` |
| 2 | `  -1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 347.` |
| 3 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.967. Support: 224.` |
| 4 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.939. Support: 337.` |
| 5 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 206.` |
| 6 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 143.` |
| 7 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 706.` |
| 8 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 565.` |
| 9 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 499.` |
| 10 | `  -1.reserved not in {(, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 261.` |
| 11 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ;, =}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {BINARY}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 919.` |
| 12 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.976. Support: 1255.` |
| 13 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.999. Support: 360.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.976. Support: 228.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.957. Support: 312.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.988. Support: 203.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 147.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.971. Support: 360.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1435.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 651.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 516.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 143.` |
| 23 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 1238.` |
| 24 | `  -1.roles in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 361.` |
| 25 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.989. Support: 225.` |
| 26 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.994. Support: 251.` |
| 27 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 153.` |
| 28 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.952. Support: 281.` |
| 29 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 662.` |
| 30 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 343.` |
| 31 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 302.` |
| 32 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 1480.` |
| 33 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.999. Support: 398.` |
| 34 | `  -1.roles not in {STRING}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1257.` |
| 35 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 211.` |
| 36 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 420.` |
| 37 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.935. Support: 286.` |
| 38 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.965. Support: 299.` |
| 39 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 330.` |
| 40 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 263.` |
| 41 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 1467.` |
| 42 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 363.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 192.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 373.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.965. Support: 297.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.956. Support: 282.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 312.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 273.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 1478.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 328.` |
| 51 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 161.` |
| 52 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.943. Support: 306.` |
| 53 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 205.` |
| 54 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 222.` |
| 55 | `  -1.reserved not in {(, {, }}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {CallExpression, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 273.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 1472.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 238.` |
| 58 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 363.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.974. Support: 211.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 186.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 156.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 163.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.972. Support: 268.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 670.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 591.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 181.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.internal_type = CallExpression<br>⇒ y = ∅<br>Confidence: 0.928. Support: 1057.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 224.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 272.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.992. Support: 188.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 418.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.954. Support: 312.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.984. Support: 282.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 341.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 281.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 1477.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 240.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.181818181818182, "max_conf": 0.9992917776107788, "max_support": 1480, "min_conf": 0.9276253581047058, "min_support": 143, "num_rules": 77}}
```
</details>
