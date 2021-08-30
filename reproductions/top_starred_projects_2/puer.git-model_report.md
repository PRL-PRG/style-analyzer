# Model report for file:///tmp/top-repos-quality-repos-ksif969j/puer.git HEAD c5ea5fc9d6f823ccbc66dfcc901a084d22f1d2ef

### Dump

```json
{'created_at': '2021-08-30 03:38:55',
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
 'size': '19.8 kB',
 'tags': [],
 'uuid': 'efe1f255-ec98-4c4b-9578-8cdb1e65328b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ksif969j/puer.git c5ea5fc9d6f823ccbc66dfcc901a084d22f1d2ef

# javascript
165 rules, avg.len. 9.4
## train
PPCR: 0.951585
### report
macro
{'f1-score': 0.7562160159305195,
 'precision': 0.7939987661997332,
 'recall': 0.7290333572723835,
 'support': 30504}
micro
{'f1-score': 0.9196498819826908,
 'precision': 0.9196498819826908,
 'recall': 0.9196498819826908,
 'support': 30504}
weighted
{'f1-score': 0.9150438877664955,
 'precision': 0.9145664757751232,
 'recall': 0.9196498819826908,
 'support': 30504}
### report_full
macro
{'f1-score': 0.7126113195850272,
 'precision': 0.7939987661997332,
 'recall': 0.6621332290272212,
 'support': 32056}
micro
{'f1-score': 0.8968350383631715,
 'precision': 0.9196498819826908,
 'recall': 0.8751247816321438,
 'support': 32056}
weighted
{'f1-score': 0.8848946494797105,
 'precision': 0.9048654067655505,
 'recall': 0.8751247816321438,
 'support': 32056}
## test
PPCR: 0.947304
### report
macro
{'f1-score': 0.6007496058263567,
 'precision': 0.7184970938122336,
 'recall': 0.5587122235310635,
 'support': 6921}
micro
{'f1-score': 0.8339835283918509,
 'precision': 0.8339835283918509,
 'recall': 0.8339835283918509,
 'support': 6921}
weighted
{'f1-score': 0.8228608901733813,
 'precision': 0.8249732842597142,
 'recall': 0.8339835283918509,
 'support': 6921}
### report_full
macro
{'f1-score': 0.5599744158793543,
 'precision': 0.7184970938122336,
 'recall': 0.507954169585226,
 'support': 7306}
micro
{'f1-score': 0.8114149153018907,
 'precision': 0.8339835283918509,
 'recall': 0.7900355871886121,
 'support': 7306}
weighted
{'f1-score': 0.7915547127733655,
 'precision': 0.8196502239946732,
 'recall': 0.7900355871886121,
 'support': 7306}
```

## javascript
### Summary
108 rules, avg.len. 9.3

| | |
|-|-|
|Min support|151|
|Max support|5157|
|Min confidence|0.9203169941902161|
|Max confidence|0.9996342062950134|

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
| 1 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 225.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 3551.` |
| 3 | `  ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 4317.` |
| 4 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.958. Support: 560.` |
| 5 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {;, {}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 3085.` |
| 6 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {STRING}<br>	∧ -3.length ≥ 4<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.956. Support: 238.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1289.` |
| 8 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.942. Support: 302.` |
| 9 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1392.` |
| 10 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 636.` |
| 11 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.949. Support: 383.` |
| 12 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 452.` |
| 13 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.roles in {BLOCK}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.990. Support: 253.` |
| 14 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label in {<newline>}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 259.` |
| 15 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |
| 16 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, MAP, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 1134.` |
| 17 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.internal_type not in {FunctionExpression}<br>	∧ ^1.roles not in {IDENTIFIER, MAP, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 656.` |
| 18 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {FunctionExpression}<br>	∧ ^1.roles in {IF} and not in {IDENTIFIER, MAP, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 440.` |
| 19 | `  ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 4222.` |
| 20 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.958. Support: 562.` |
| 21 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +2.internal_type = CommentBlock<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.997. Support: 179.` |
| 22 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {;, {}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 3045.` |
| 23 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {CALL, STRING}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.923. Support: 306.` |
| 24 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1299.` |
| 25 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.931. Support: 325.` |
| 26 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1352.` |
| 27 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 368.` |
| 28 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 586.` |
| 29 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 451.` |
| 30 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.roles in {BLOCK}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.993. Support: 217.` |
| 31 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label in {<newline>}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 260.` |
| 32 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 1144.` |
| 33 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 176.` |
| 34 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {IF, LITERAL, OPERATOR}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 529.` |
| 35 | `  ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 4291.` |
| 36 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.950. Support: 533.` |
| 37 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 3027.` |
| 38 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {STRING} and not in {IDENTIFIER}<br>	∧ -3.length ≥ 4<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.944. Support: 296.` |
| 39 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1272.` |
| 40 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1294.` |
| 41 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 639.` |
| 42 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.957. Support: 361.` |
| 43 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 2073.` |
| 44 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 652.` |
| 45 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {BINARY}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 151.` |
| 46 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {;, {}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 3094.` |
| 47 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {CALL, STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.956. Support: 241.` |
| 48 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 1283.` |
| 49 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1378.` |
| 50 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.954. Support: 384.` |
| 51 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 604.` |
| 52 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.934. Support: 569.` |
| 53 | `  -1.reserved not in {;}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 4226.` |
| 54 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 3053.` |
| 55 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {ARGUMENT} and not in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.941. Support: 262.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1287.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.947. Support: 333.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1432.` |
| 59 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 357.` |
| 60 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 615.` |
| 61 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 488.` |
| 62 | `  •••start_col ≥ 5<br>	∧ -1.internal_type = CommentBlock<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 237.` |
| 63 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {CommentBlock, CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 204.` |
| 64 | `  •••start_col ≤ 4<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ -2.diff_offset ≥ 14<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.979. Support: 217.` |
| 65 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {CALL} and not in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.952. Support: 280.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1367.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 648.` |
| 68 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 363.` |
| 69 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 3470.` |
| 70 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 1375.` |
| 71 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.939. Support: 354.` |
| 72 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 1289.` |
| 73 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 630.` |
| 74 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {CALL, STRING} and not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.935. Support: 269.` |
| 75 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ -2.internal_type = CommentBlock<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.980. Support: 273.` |
| 76 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {IDENTIFIER, STRING}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 469.` |
| 77 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION, IDENTIFIER, STRING}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {MAP, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 234.` |
| 78 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER, STRING}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {MAP, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 209.` |
| 79 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER, STRING}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -4.length ≤ 1<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = FunctionExpression<br>	∧ ^1.roles not in {IDENTIFIER, MAP, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 155.` |
| 80 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 3103.` |
| 81 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {ARGUMENT, STRING} and not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.937. Support: 279.` |
| 82 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1264.` |
| 83 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1388.` |
| 84 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 633.` |
| 85 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 387.` |
| 86 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER, STRING}<br>	∧ -2.roles in {BLOCK}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.984. Support: 220.` |
| 87 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER, STRING}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label in {<newline>}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 247.` |
| 88 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER, STRING}<br>	∧ -2.label in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 1119.` |
| 89 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 188.` |
| 90 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 713.` |
| 91 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER, STRING}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved not in {(}<br>	∧ -2.roles not in {BLOCK}<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {IF} and not in {LITERAL, OPERATOR}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 418.` |
| 92 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 1430.` |
| 93 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.920. Support: 283.` |
| 94 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1308.` |
| 95 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 390.` |
| 96 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 606.` |
| 97 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.length ≥ 4<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.965. Support: 240.` |
| 98 | `  •••start_col ≥ 5<br>	∧ -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 162.` |
| 99 | `  -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 924.` |
| 100 | `  -1.reserved not in {;, {}<br>	∧ -1.roles in {STRING}<br>	∧ -3.length ≥ 4<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.946. Support: 232.` |
| 101 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 506.` |
| 102 | `  -1.reserved = if<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 385.` |
| 103 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 250.` |
| 104 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, if}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 156.` |
| 105 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 623.` |
| 106 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -1.length ≥ 11<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 229.` |
| 107 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<newline>} and not in {<-space>}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 190.` |
| 108 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<-space>, <newline>}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 5157.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.287037037037036, "max_conf": 0.9996342062950134, "max_support": 5157, "min_conf": 0.9203169941902161, "min_support": 151, "num_rules": 108}}
```
</details>
