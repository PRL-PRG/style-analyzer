# Model report for file:///tmp/top-repos-quality-repos-91ekfjqr/sigmapi-web.git HEAD 6cd7c2f0da8b3a6cea9cce308fa377857436947d

### Dump

```json
{'created_at': '2021-08-22 07:17:04',
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
 'size': '23.5 kB',
 'tags': [],
 'uuid': '9d12f584-71b7-423d-b6d1-9b726692ca19',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-91ekfjqr/sigmapi-web.git 6cd7c2f0da8b3a6cea9cce308fa377857436947d

# javascript
53 rules, avg.len. 9.5
## train
PPCR: 0.911275
### report
macro
{'f1-score': 0.5355374789236408,
 'precision': 0.5670111084674688,
 'recall': 0.5161402973815992,
 'support': 159854}
micro
{'f1-score': 0.965312097288776,
 'precision': 0.965312097288776,
 'recall': 0.965312097288776,
 'support': 159854}
weighted
{'f1-score': 0.9616253842173824,
 'precision': 0.9591558846563468,
 'recall': 0.965312097288776,
 'support': 159854}
### report_full
macro
{'f1-score': 0.4387026039759924,
 'precision': 0.5670111084674688,
 'recall': 0.38623697681761343,
 'support': 175418}
micro
{'f1-score': 0.9205003698489584,
 'precision': 0.965312097288776,
 'recall': 0.8796645726208256,
 'support': 175418}
weighted
{'f1-score': 0.9013426996870395,
 'precision': 0.9456475249627856,
 'recall': 0.8796645726208256,
 'support': 175418}
## test
PPCR: 0.884385
### report
macro
{'f1-score': 0.48203276474138906,
 'precision': 0.5781196942305982,
 'recall': 0.4660007846501404,
 'support': 20202}
micro
{'f1-score': 0.9548064548064548,
 'precision': 0.9548064548064548,
 'recall': 0.9548064548064548,
 'support': 20202}
weighted
{'f1-score': 0.9488004533049613,
 'precision': 0.9537258826299087,
 'recall': 0.9548064548064548,
 'support': 20202}
### report_full
macro
{'f1-score': 0.3934727656160949,
 'precision': 0.5781196942305982,
 'recall': 0.3549773262331938,
 'support': 22843}
micro
{'f1-score': 0.8962248809385527,
 'precision': 0.9548064548064548,
 'recall': 0.8444162325438865,
 'support': 22843}
weighted
{'f1-score': 0.8703480129355534,
 'precision': 0.9396754179722882,
 'recall': 0.8444162325438865,
 'support': 22843}
```

## javascript
### Summary
34 rules, avg.len. 9.6

| | |
|-|-|
|Min support|133|
|Max support|23979|
|Min confidence|0.9211981296539307|
|Max confidence|0.9996615052223206|

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
                     'min_samples_leaf': 120,
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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 23979.` |
| 2 | `  -1.label in {<space>}<br>	∧ ^1.roles in {OPERATOR} and not in {ARITHMETIC, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.956. Support: 488.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles in {STRING}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.988. Support: 200.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 689.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 518.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 380.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 6<br>	∧ -4.length ≤ 30<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 13328.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -4.length ≤ 30<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 1631.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -4.length ≤ 30<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 342.` |
| 10 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 2819.` |
| 11 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {+}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.923. Support: 1932.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 292.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.label in {<newline>}<br>	∧ +1.reserved not in {=, }}<br>	∧ +2.roles not in {FUNCTION}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 429.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, FUNCTION}<br>	∧ -4.label not in {<newline>}<br>	∧ -4.length ≥ 2<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 407.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, FUNCTION}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.reserved not in {(, =, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 2537.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION} and not in {FUNCTION}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 19568.` |
| 17 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 9199.` |
| 18 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4431.` |
| 19 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 2126.` |
| 20 | `  -1.internal_type = CommentBlock<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 17<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 828.` |
| 21 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 16<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.internal_type not in {CommentLine}<br>	∧ +2.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.974. Support: 588.` |
| 22 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 16<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 241.` |
| 23 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -4.diff_offset ≥ 13<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 1329.` |
| 24 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ -2.diff_offset ≤ 16<br>	∧ -4.diff_offset ≤ 12<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 149.` |
| 25 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 1472.` |
| 26 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.921. Support: 1085.` |
| 27 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 929.` |
| 28 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +2.roles in {NAME}<br>	∧ ^1.internal_type not in {File, IfStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.992. Support: 198.` |
| 29 | `  •••start_col ≤ 9<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.981. Support: 133.` |
| 30 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type = VariableDeclaration<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 186.` |
| 31 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -5.label in {<newline>}<br>	∧ +1.reserved not in {,, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 256.` |
| 32 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -3.label in {<newline>}<br>	∧ -3.reserved not in {=}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.961. Support: 193.` |
| 33 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.label in {<newline>}<br>	∧ -3.reserved not in {=}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1223.` |
| 34 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {=}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, :, ;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 18729.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.588235294117647, "max_conf": 0.9996615052223206, "max_support": 23979, "min_conf": 0.9211981296539307, "min_support": 133, "num_rules": 34}}
```
</details>