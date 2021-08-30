# Model report for file:///tmp/top-repos-quality-repos-s4oh_2zs/snowpack.git HEAD d8cc54a7816faef9d98459a1e24f54912163868b

### Dump

```json
{'created_at': '2021-08-29 14:00:39',
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
 'size': '19.5 kB',
 'tags': [],
 'uuid': '631ec96f-c491-4701-b577-38a193cec6f8',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-s4oh_2zs/snowpack.git d8cc54a7816faef9d98459a1e24f54912163868b

# javascript
137 rules, avg.len. 8.3
## train
PPCR: 0.981099
### report
macro
{'f1-score': 0.7321274073695692,
 'precision': 0.77766158401727,
 'recall': 0.7054357149469686,
 'support': 46664}
micro
{'f1-score': 0.932303274472827,
 'precision': 0.932303274472827,
 'recall': 0.932303274472827,
 'support': 46664}
weighted
{'f1-score': 0.9276999880578296,
 'precision': 0.926284515163535,
 'recall': 0.932303274472827,
 'support': 46664}
### report_full
macro
{'f1-score': 0.7088132842691374,
 'precision': 0.77766158401727,
 'recall': 0.6740847212796637,
 'support': 47563}
micro
{'f1-score': 0.9234083649060247,
 'precision': 0.932303274472827,
 'recall': 0.9146815802199189,
 'support': 47563}
weighted
{'f1-score': 0.9165732105429434,
 'precision': 0.9250644272590365,
 'recall': 0.9146815802199189,
 'support': 47563}
## test
PPCR: 0.984544
### report
macro
{'f1-score': 0.6682222189206211,
 'precision': 0.724025588539087,
 'recall': 0.6535142746674574,
 'support': 13504}
micro
{'f1-score': 0.9052873222748815,
 'precision': 0.9052873222748815,
 'recall': 0.9052873222748815,
 'support': 13504}
weighted
{'f1-score': 0.898535455436193,
 'precision': 0.8980646293890383,
 'recall': 0.9052873222748815,
 'support': 13504}
### report_full
macro
{'f1-score': 0.6524620333812114,
 'precision': 0.724025588539087,
 'recall': 0.6348987192199179,
 'support': 13716}
micro
{'f1-score': 0.8982365907421014,
 'precision': 0.9052873222748815,
 'recall': 0.8912948381452318,
 'support': 13716}
weighted
{'f1-score': 0.889111111504666,
 'precision': 0.8968091047234187,
 'recall': 0.8912948381452318,
 'support': 13716}
```

## javascript
### Summary
76 rules, avg.len. 8.8

| | |
|-|-|
|Min support|148|
|Max support|14525|
|Min confidence|0.9201030731201172|
|Max confidence|0.9996835589408875|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.960. Support: 1987.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.979. Support: 165.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.993. Support: 219.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.931. Support: 542.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 348.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 149.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 830.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 751.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.993. Support: 754.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 414.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 196.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = async<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 169.` |
| 13 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 179.` |
| 14 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 4466.` |
| 15 | `  •••start_col ≥ 25<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 224.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 14227.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type = ObjectExpression<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 149.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 276.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.944. Support: 577.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.931. Support: 750.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1580.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 8581.` |
| 23 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 1953.` |
| 24 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 417.` |
| 25 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 148.` |
| 26 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 850.` |
| 27 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.933. Support: 846.` |
| 28 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.997. Support: 763.` |
| 29 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 186.` |
| 30 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 3<br>	∧ -4.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 348.` |
| 31 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 3<br>	∧ -4.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 421.` |
| 32 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 9778.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +2.reserved = :<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 164.` |
| 34 | `  •••start_col ≥ 34<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 194.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ +5.length = 0<br>	∧ ^1.internal_type not in {File}<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 175.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 976.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 640.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 427.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 325.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |
| 41 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 16<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 149.` |
| 42 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 16<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 383.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 835.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 332.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 555.` |
| 46 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 151.` |
| 47 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 255.` |
| 48 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1381.` |
| 49 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<newline>}<br>	∧ -3.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 497.` |
| 50 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<newline>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 384.` |
| 51 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -2.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 13364.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.internal_type not in {StringLiteral}<br>	∧ +5.length = 0<br>	∧ ^1.internal_type not in {File}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 168.` |
| 53 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 213.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 1963.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.999. Support: 362.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 148.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 870.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.961. Support: 784.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.992. Support: 726.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 194.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 11169.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {BINARY}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1520.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 894.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 242.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 224.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1475.` |
| 67 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT} and not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 372.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {KEY}<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 200.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.979. Support: 8377.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.941. Support: 819.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 292.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, {}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 155.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 10050.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.roles not in {LITERAL}<br>	∧ +4.length = 0<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 193.` |
| 75 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles in {FUNCTION, STATEMENT} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 150.` |
| 76 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 14525.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.789473684210526, "max_conf": 0.9996835589408875, "max_support": 14525, "min_conf": 0.9201030731201172, "min_support": 148, "num_rules": 76}}
```
</details>
